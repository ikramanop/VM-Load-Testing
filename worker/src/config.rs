use envconfig::Envconfig;

#[derive(Envconfig, Clone)]
pub(crate) struct Config {
    #[envconfig(from = "KAFKA_HOST")]
    pub(crate) kafka_host: String,

    #[envconfig(nested = true)]
    pub(crate) disk: DiskConfig,

    #[envconfig(nested = true)]
    pub(crate) cpu: CpuConfig,

    #[envconfig(nested = true)]
    pub(crate) ram: RamConfig,

    #[envconfig(nested = true)]
    pub(crate) dummy: DummyConfig,
}

pub(crate) trait WorkerConfig: Sync {
    fn kafka_topic(&self) -> String;

    fn kafka_group_id(&self) -> String;
}

#[derive(Envconfig, Clone)]
pub(crate) struct DiskConfig {
    #[envconfig(from = "DISK_META")]
    pub(crate) meta: String,

    #[envconfig(from = "DISK_KAFKA_TOPIC")]
    pub(crate) kafka_topic: String,

    #[envconfig(from = "DISK_KAFKA_GROUP_ID")]
    pub(crate) kafka_group_id: String,
}

impl WorkerConfig for DiskConfig {
    fn kafka_topic(&self) -> String {
        self.kafka_topic.clone()
    }

    fn kafka_group_id(&self) -> String {
        self.kafka_group_id.clone()
    }
}

#[derive(Envconfig, Clone)]
pub(crate) struct CpuConfig {
    #[envconfig(from = "CPU_META")]
    pub(crate) meta: String,

    #[envconfig(from = "CPU_KAFKA_TOPIC")]
    pub(crate) kafka_topic: String,

    #[envconfig(from = "CPU_KAFKA_GROUP_ID")]
    pub(crate) kafka_group_id: String,
}

impl WorkerConfig for CpuConfig {
    fn kafka_topic(&self) -> String {
        self.kafka_topic.clone()
    }

    fn kafka_group_id(&self) -> String {
        self.kafka_group_id.clone()
    }
}

#[derive(Envconfig, Clone)]
pub(crate) struct RamConfig {
    #[envconfig(from = "RAM_META")]
    pub(crate) meta: String,

    #[envconfig(from = "RAM_KAFKA_TOPIC")]
    pub(crate) kafka_topic: String,

    #[envconfig(from = "RAM_KAFKA_GROUP_ID")]
    pub(crate) kafka_group_id: String,
}

impl WorkerConfig for RamConfig {
    fn kafka_topic(&self) -> String {
        self.kafka_topic.clone()
    }

    fn kafka_group_id(&self) -> String {
        self.kafka_group_id.clone()
    }
}

#[derive(Envconfig, Clone)]
pub(crate) struct DummyConfig {
    #[envconfig(from = "DUMMY_META")]
    pub(crate) meta: String,

    #[envconfig(from = "DUMMY_KAFKA_TOPIC")]
    pub(crate) kafka_topic: String,

    #[envconfig(from = "DUMMY_KAFKA_GROUP_ID")]
    pub(crate) kafka_group_id: String,
}

impl WorkerConfig for DummyConfig {
    fn kafka_topic(&self) -> String {
        self.kafka_topic.clone()
    }

    fn kafka_group_id(&self) -> String {
        self.kafka_group_id.clone()
    }
}
