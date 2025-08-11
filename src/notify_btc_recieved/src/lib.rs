use std::sync::mpsc::{Sender, Receiver, channel};

/// Notification message for BTC received
pub struct BtcReceivedNotification {
    pub amount: f64,
    pub from_address: String,
    pub tx_id: String,
}

/// Notifier for BTC received events
pub struct BtcNotifier {
    sender: Sender<BtcReceivedNotification>,
}

impl BtcNotifier {
    /// Creates a new notifier and returns it along with its receiver
    pub fn new() -> (Self, Receiver<BtcReceivedNotification>) {
        let (sender, receiver) = channel();
        (Self { sender }, receiver)
    }

    /// Send a notification
    pub fn notify(&self, amount: f64, from_address: String, tx_id: String) {
        let notification = BtcReceivedNotification {
            amount,
            from_address,
            tx_id,
        };
        let _ = self.sender.send(notification);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_btc_notifier() {
        let (notifier, receiver) = BtcNotifier::new();
        notifier.notify(0.5, "1BitcoinAddr".to_string(), "tx123".to_string());

        let received = receiver.recv().unwrap();
        assert_eq!(received.amount, 0.5);
        assert_eq!(received.from_address, "1BitcoinAddr");
        assert_eq!(received.tx_id, "tx123");
    }
}