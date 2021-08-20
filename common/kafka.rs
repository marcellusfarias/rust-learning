use rdkafka::config::{ClientConfig, RDKafkaLogLevel};
use rdkafka::consumer::StreamConsumer;
use rdkafka::error::KafkaResult;
use rdkafka::client::ClientContext;
use rdkafka::topic_partition_list::TopicPartitionList;
use rdkafka::consumer::{Consumer, ConsumerContext, Rebalance};
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::util::Timeout;


// A context can be used to change the behavior of producers and consumers by adding callbacks
// that will be executed by librdkafka.
// This particular context sets up custom callbacks to log rebalancing events.
pub struct CustomContext;

impl ClientContext for CustomContext {}

impl ConsumerContext for CustomContext {
    fn pre_rebalance(&self, rebalance: &Rebalance) {
        // info!("Pre rebalance {:?}", rebalance);
    }

    fn post_rebalance(&self, rebalance: &Rebalance) {
        // info!("Post rebalance {:?}", rebalance);
    }

    fn commit_callback(&self, result: KafkaResult<()>, _offsets: &TopicPartitionList) {
        // info!("Committing offsets: {:?}", result);
    }
}

// A type alias with your custom consumer can be created for convenience.
type LoggingConsumer = StreamConsumer<CustomContext>;

pub fn create_consumer() -> LoggingConsumer {
    let context = CustomContext;

    // Initialize variables from environment

    let consumer: LoggingConsumer = ClientConfig::new()
        .set("group.id", "dc-sender")
        .set("bootstrap.servers", "kafka1:19091,kafka2:19092,kafka3:19093")
        .set("enable.partition.eof", "false")
        .set("heartbeat.interval.ms", "2000") // New. Should be 1/3 of session.timeout.ms
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "false") // Changed from true so consumer controls
        //        .set("auto.commit.interval.ms", "5000")
        .set("enable.auto.offset.store", "false")
        // TODO Remove the log level to debug line once development is over. That or set an environment variable to hold this.
        .set_log_level(RDKafkaLogLevel::Debug)
        .create_with_context(context)
        .expect("Consumer creation failed"); //TODO expect ok, or do we handle?

    consumer
        .subscribe(&["dc-msgs"])
        .expect("Can't subscribe to specified topics"); //TODO is this expect ok, or do we try to handle it?

    consumer
}
