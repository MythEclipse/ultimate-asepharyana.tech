//! Email queue system for background email processing
//! Prevents blocking API responses while sending emails

use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::{error, info};

use super::email::{EmailService, EmailTemplate};

/// Email queue message
#[derive(Debug, Clone)]
pub enum EmailQueueMessage {
    SendEmail(EmailTemplate),
    Shutdown,
}

/// Email queue for background processing
#[derive(Clone)]
pub struct EmailQueue {
    sender: mpsc::UnboundedSender<EmailQueueMessage>,
}

impl EmailQueue {
    /// Create a new email queue and start the background worker
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::unbounded_channel();

        // Spawn background worker
        tokio::spawn(Self::worker(receiver));

        info!("📬 Email queue initialized");

        Self { sender }
    }

    /// Send email asynchronously (non-blocking)
    pub fn send(&self, template: EmailTemplate) -> Result<(), String> {
        self.sender
            .send(EmailQueueMessage::SendEmail(template))
            .map_err(|e| format!("Failed to queue email: {}", e))
    }

    /// Shutdown the email queue
    pub fn shutdown(&self) -> Result<(), String> {
        self.sender
            .send(EmailQueueMessage::Shutdown)
            .map_err(|e| format!("Failed to send shutdown signal: {}", e))
    }

    /// Background worker that processes emails
    async fn worker(mut receiver: mpsc::UnboundedReceiver<EmailQueueMessage>) {
        let email_service = Arc::new(EmailService::new());

        info!("📨 Email worker started");

        while let Some(message) = receiver.recv().await {
            match message {
                EmailQueueMessage::SendEmail(template) => {
                    let service = Arc::clone(&email_service);
                    let to_email = template.to_email.clone();
                    let subject = template.subject.clone();

                    // Spawn separate task for each email to avoid blocking
                    tokio::spawn(async move {
                        match service.send_email_internal(template).await {
                            Ok(_) => {
                                info!("✅ Email sent: {} - '{}'", to_email, subject);
                            }
                            Err(e) => {
                                error!("❌ Failed to send email to {}: {}", to_email, e);
                            }
                        }
                    });
                }
                EmailQueueMessage::Shutdown => {
                    info!("📪 Email worker shutting down");
                    break;
                }
            }
        }

        info!("📭 Email worker stopped");
    }
}

impl Default for EmailQueue {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper methods for EmailService to work with queue
impl EmailService {
    /// Internal method called by the queue worker
    pub async fn send_email_internal(
        &self,
        template: EmailTemplate,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // This reuses the send_email method but wraps errors
        self.send_email(template)
            .await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
    }
}
