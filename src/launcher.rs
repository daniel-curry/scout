use gio::AppInfo;
use gio::prelude::AppInfoExt;
use glib::{Cast, SpawnFlags};
use crate::entry::SystemAction;

pub fn needs_terminal(app: &AppInfo) -> bool {
    if let Some(dai) = app.downcast_ref::<gio::DesktopAppInfo>() {
        return dai.boolean("Terminal");
    }
    false
}

pub fn launch_gui_app(app: &AppInfo) -> Result<(), String> {
    let ctx = gio::AppLaunchContext::new();

    // Prefer DesktopAppInfo so we can inject a child-setup hook (setsid).
    if let Some(dai) = app.dynamic_cast_ref::<gio::DesktopAppInfo>() {
        // No URIs/files to pass
        let uris: [&str; 0] = [];

        let spawn_flags =
            SpawnFlags::SEARCH_PATH
                | SpawnFlags::STDOUT_TO_DEV_NULL
                | SpawnFlags::STDERR_TO_DEV_NULL;

        // Called after fork() but before exec() in the child.
        let user_setup: Option<Box<dyn FnOnce()>> = Some(Box::new(|| {
            #[cfg(unix)]
            unsafe {
                let _ = libc::setsid();
            }
        }));

        dai.launch_uris_as_manager(&uris, Some(&ctx), spawn_flags, user_setup, None)
            .map_err(|e| format!("Failed to launch app '{}': {}", app.name(), e))?;

        return Ok(());
    }
    Ok(())
}

pub fn launch_terminal_application(app_argv: &[String], terminal_argv_prefix: &[String]) -> Result<(), glib::Error> {

    // Build argv = terminal + exec-flag/args + app argv
    let mut argv: Vec<String> = Vec::new();
    argv.extend_from_slice(terminal_argv_prefix);
    argv.extend_from_slice(app_argv);

    // Convert to &OsStr slices as gtk-rs expects
    let argv_os: Vec<std::ffi::OsString> = argv.into_iter().map(Into::into).collect();
    let argv_refs: Vec<&std::ffi::OsStr> = argv_os.iter().map(|s| s.as_os_str()).collect();

    let launcher = gio::SubprocessLauncher::new(gio::SubprocessFlags::NONE);

    // setsid() child setup: detach from the launcher's session.
    launcher.set_child_setup(|| {
        #[cfg(unix)]
        unsafe {
            let _ = libc::setsid();
        }
    });

    // Spawn and immediately drop handle.
    // GSubprocess reaps children quickly to avoid zombies.
    let _child = launcher.spawn(&argv_refs)?;
    Ok(())
}

pub fn launch_system_action(action: &SystemAction) -> Result<(), String> {
    match action {
        SystemAction::Shutdown => system_shutdown::shutdown(),
        SystemAction::Restart => system_shutdown::reboot(),
        SystemAction::Hibernate => system_shutdown::hibernate(),
        SystemAction::Sleep => system_shutdown::sleep(),
    }.expect("System action failed");
    
    Ok(())
}
