#[derive(clap::Parser)]
#[clap(name = "lt-worker")]
#[clap(bin_name = "lt-worker")]
pub(crate) enum CMD {
    Disk,
    Cpu,
    Ram,
    Dummy,
}
