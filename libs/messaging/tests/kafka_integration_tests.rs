use messaging::kafka::KafkaBroker;
use messaging::pubsub::{MessageBroker, PubSubMessage};
use std::sync::{Arc, Mutex};
use tokio::sync::Notify;
use tokio::time::{sleep, timeout, Duration};
use uuid::Uuid;

const KAFKA_BOOTSTRAP_SERVERS: &str = "localhost:9092";
const GROUP_ID: &str = "test-group";
const TEST_TOPIC: &str = "test-topic";

async fn create_test_broker() -> KafkaBroker {
    KafkaBroker::new(KAFKA_BOOTSTRAP_SERVERS, GROUP_ID).await
}

#[tokio::test]
#[ignore]
async fn test_create_and_delete_topic() {
    let broker = create_test_broker().await;

    let topic_name = format!("{}-{}", TEST_TOPIC, Uuid::new_v4());
    // Create topic
    broker
        .create_topic(&topic_name)
        .await
        .expect("Failed to create topic");

    // Wait for topic creation
    sleep(Duration::from_secs(1)).await;

    // List topics
    let topics = broker.list_topics().await.expect("Failed to list topics");
    assert!(topics.contains(&topic_name.to_string()));

    // Delete topic
    broker
        .delete_topic(&topic_name)
        .await
        .expect("Failed to delete topic");

    // Wait for topic deletion
    sleep(Duration::from_secs(1)).await;

    // List topics again
    let topics = broker.list_topics().await.expect("Failed to list topics");
    assert!(!topics.contains(&topic_name.to_string()));
}

#[tokio::test]
#[ignore]
async fn test_publish_and_subscribe() {
    let broker = create_test_broker().await;
    let topic_name = format!("{}-{}", TEST_TOPIC, Uuid::new_v4());

    // Create topic
    broker
        .create_topic(&topic_name)
        .await
        .expect("Failed to create topic");

    // Wait for topic creation
    sleep(Duration::from_secs(1)).await;

    // Prepare test message
    let test_message = PubSubMessage {
        key: Some(b"test-key".to_vec()),
        payload: b"test-payload".to_vec(),
    };

    // Shared state to store the received message
    let received_message = Arc::new(Mutex::new(None));

    // Notify instance to signal when the message is received
    let notify = Arc::new(Notify::new());

    // Clones for the closure
    let received_message_clone = Arc::clone(&received_message);
    let notify_clone = Arc::clone(&notify);

    // Subscribe to the topic
    broker
        .subscribe(&topic_name, move |msg| {
            let received_message_clone = Arc::clone(&received_message_clone);
            let notify_clone = Arc::clone(&notify_clone);
            async move {
                // Store the received message
                let mut received_message_lock = received_message_clone.lock().unwrap();
                *received_message_lock = Some(msg);
                // Notify the test that the message has been received
                notify_clone.notify_one();
            }
        })
        .await
        .expect("Failed to subscribe");

    // Publish the message
    broker
        .publish(&topic_name, test_message.clone())
        .await
        .expect("Failed to publish message");

    // Wait for the message to be received or timeout after 5 seconds
    let result = timeout(Duration::from_secs(5), notify.notified()).await;
    assert!(result.is_ok(), "Timed out waiting for message");

    // Access the received message from the shared state
    let received_message_lock = received_message.lock().unwrap();
    let received_message = received_message_lock.as_ref().expect("No message received");

    // Assert that the received message matches the sent message
    assert_eq!(received_message.key, test_message.key);
    assert_eq!(received_message.payload, test_message.payload);

    // Clean up
    broker
        .delete_topic(&topic_name)
        .await
        .expect("Failed to delete topic");
}
