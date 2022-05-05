//! A simple render.

use color_eyre::eyre::Result;

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use tracing::info;

use std::fs::File;
use std::io::{BufWriter, Write};

use lib::util::write_color;

static H: u16 = 256;
static W: u16 = 256;

#[tokio::main]
async fn main() -> Result<()> {
    setup_logging()?;

    // Construct the buffer that we write out to.
    let mut buffer = BufWriter::new(File::create("./image.ppm")?);
    info!("Succesfully established buffer.");

    // Write the first few bytes: the PPM header.
    write!(buffer, "P3\n{} {}\n255\n", W, H)?;
    info!("Wrote PPM header.");

    // Set up progress bars, so we know where we are.
    let (bars, x_bar, y_bar) = setup_bars();

    let _ = tokio::spawn(async move {
        for x in (0..H).rev() {
            x_bar.inc(1);
            for y in 0..W {
                y_bar.inc(1);

                let b = (0.25 * 255.) as u16;

                write_color(&mut buffer, y, x, b).unwrap();
            }
            y_bar.reset();
        }
    });

    bars.join().unwrap();

    Ok(())
}

/// Set up tracing and eyre.
fn setup_logging() -> Result<()> {
    use tracing_error::ErrorLayer;
    use tracing_subscriber::prelude::*;
    use tracing_subscriber::{fmt, EnvFilter};

    // Set up tracing.
    let fmt_layer = fmt::layer().with_target(false);
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .unwrap();

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .with(ErrorLayer::default())
        .init();

    // Set up hooks.
    let (panic_hook, eyre_hook) = color_eyre::config::HookBuilder::default().into_hooks();

    eyre_hook.install()?;

    std::panic::set_hook(Box::new(move |pi| {
        tracing::error!("{}", panic_hook.panic_report(pi));
    }));

    Ok(())
}

/// Set up the progress bar indicators.
fn setup_bars() -> (MultiProgress, ProgressBar, ProgressBar) {
    let bars = MultiProgress::new();
    let bar_style = ProgressStyle::default_bar()
        .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
        .progress_chars("##-");

    let x_bar = bars.add(ProgressBar::new(W as u64));
    let y_bar = bars.add(ProgressBar::new(H as u64));

    x_bar.set_style(bar_style.clone());
    y_bar.set_style(bar_style.clone());

    x_bar.tick();
    y_bar.tick();

    (bars, x_bar, y_bar)
}
