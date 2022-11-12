#[derive(clap::Parser)]
#[clap(name = "ltworker")]
#[clap(bin_name = "ltworker")]
pub(crate) enum CMD {
    Disk,
    Cpu,
    Ram,
    Dummy,
}
