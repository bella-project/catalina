// Copyright 2022-2025 the Catalina & Vello Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::time::Duration;

use anyhow::Result;
use notify_debouncer_full::notify::RecursiveMode;
use notify_debouncer_full::{new_debouncer, DebounceEventResult};

pub fn hot_reload(mut f: impl FnMut() -> Option<()> + Send + 'static) -> Result<impl Sized> {
    let mut debouncer = new_debouncer(
        Duration::from_millis(500),
        None,
        move |res: DebounceEventResult| match res {
            Ok(events) => {
                for event in events {
                    // Don't hot reload if the file was only read (i.e. by us...)
                    if !matches!(
                        event.kind,
                        notify_debouncer_full::notify::EventKind::Access(_)
                    ) {
                        f().unwrap();
                        break;
                    }
                }
            }
            Err(e) => println!("Hot reloading file watching failed: {e:?}"),
        },
    )?;

    debouncer.watch(
        catalina_shaders::compile::shader_dir().as_path(),
        // We currently don't support hot reloading the imports, so don't recurse into there
        RecursiveMode::NonRecursive,
    )?;
    Ok(debouncer)
}
