use crate::config::CONFIG_MAP;
use crate::utils::error::AppError;
use serde::{Deserialize, Serialize};

/// Email service configuration
#[derive(Debug, Clone)]
pub struct EmailConfig {
    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_username: String,
    pub smtp_password: String,
    pub smtp_secure: bool, // true for SSL (port 465), false for STARTTLS (port 587)
    pub from_email: String,
    pub from_name: String,
}

impl EmailConfig {
    pub fn from_env() -> Self {
        EmailConfig {
            smtp_host: CONFIG_MAP
                .get("MAIL_HOST")
                .or_else(|| CONFIG_MAP.get("SMTP_HOST"))
                .cloned()
                .unwrap_or_else(|| "smtp.gmail.com".to_string()),
            smtp_port: CONFIG_MAP
                .get("MAIL_PORT")
                .or_else(|| CONFIG_MAP.get("SMTP_PORT"))
                .and_then(|p| p.parse().ok())
                .unwrap_or(587),
            smtp_username: CONFIG_MAP
                .get("MAIL_USER")
                .or_else(|| CONFIG_MAP.get("SMTP_USERNAME"))
                .cloned()
                .unwrap_or_else(|| "noreply@example.com".to_string()),
            smtp_password: CONFIG_MAP
                .get("MAIL_PASSWORD")
                .or_else(|| CONFIG_MAP.get("SMTP_PASSWORD"))
                .cloned()
                .unwrap_or_default(),
            smtp_secure: CONFIG_MAP
                .get("MAIL_SECURE")
                .and_then(|v| v.parse().ok())
                .unwrap_or(false),
            from_email: CONFIG_MAP
                .get("FROM_EMAIL")
                .cloned()
                .unwrap_or_else(|| "noreply@example.com".to_string()),
            from_name: CONFIG_MAP
                .get("FROM_NAME")
                .cloned()
                .unwrap_or_else(|| "Auth System".to_string()),
        }
    }
}

/// Email template data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailTemplate {
    pub to_email: String,
    pub to_name: Option<String>,
    pub subject: String,
    pub html_body: String,
    pub text_body: String,
}

/// Email service implementation
pub struct EmailService {
    #[allow(dead_code)]
    config: EmailConfig,
    app_url: String,
}

impl EmailService {
    pub fn new() -> Self {
        let app_url = CONFIG_MAP
            .get("APP_URL")
            .cloned()
            .unwrap_or_else(|| "http://localhost:3000".to_string());

        EmailService {
            config: EmailConfig::from_env(),
            app_url,
        }
    }

    /// Send verification email
    pub async fn send_verification_email(
        &self,
        to_email: &str,
        to_name: &str,
        verification_token: &str,
    ) -> Result<(), AppError> {
        let verification_url = format!("{}/auth/verify?token={}", self.app_url, verification_token);

        let html_body = format!(
            r#"
            <!DOCTYPE html>
            <html>
            <head>
                <style>
                    body {{ font-family: Arial, sans-serif; line-height: 1.6; color: #333; }}
                    .container {{ max-width: 600px; margin: 0 auto; padding: 20px; }}
                    .header {{ background-color: #4CAF50; color: white; padding: 20px; text-align: center; }}
                    .content {{ padding: 20px; background-color: #f9f9f9; }}
                    .button {{ display: inline-block; padding: 12px 24px; background-color: #4CAF50; color: white; text-decoration: none; border-radius: 4px; margin: 20px 0; }}
                    .footer {{ text-align: center; padding: 20px; color: #666; font-size: 12px; }}
                </style>
            </head>
            <body>
                <div class="container">
                    <div class="header">
                        <h1>Email Verification</h1>
                    </div>
                    <div class="content">
                        <h2>Hello {}!</h2>
                        <p>Thank you for registering with us. Please verify your email address to complete your registration.</p>
                        <p>Click the button below to verify your email:</p>
                        <a href="{}" class="button">Verify Email</a>
                        <p>Or copy and paste this link into your browser:</p>
                        <p style="word-break: break-all;">{}</p>
                        <p>This link will expire in 24 hours.</p>
                        <p>If you didn't create an account, you can safely ignore this email.</p>
                    </div>
                    <div class="footer">
                        <p>&copy; 2025 Auth System. All rights reserved.</p>
                    </div>
                </div>
            </body>
            </html>
            "#,
            to_name, verification_url, verification_url
        );

        let text_body = format!(
            "Hello {}!\n\nThank you for registering. Please verify your email by visiting:\n{}\n\nThis link will expire in 24 hours.\n\nIf you didn't create an account, you can safely ignore this email.",
            to_name, verification_url
        );

        let template = EmailTemplate {
            to_email: to_email.to_string(),
            to_name: Some(to_name.to_string()),
            subject: "Verify Your Email Address".to_string(),
            html_body,
            text_body,
        };

        self.send_email(template).await
    }

    /// Send password reset email
    pub async fn send_password_reset_email(
        &self,
        to_email: &str,
        to_name: &str,
        reset_token: &str,
    ) -> Result<(), AppError> {
        let reset_url = format!("{}/auth/reset-password?token={}", self.app_url, reset_token);

        let html_body = format!(
            r#"
            <!DOCTYPE html>
            <html>
            <head>
                <style>
                    body {{ font-family: Arial, sans-serif; line-height: 1.6; color: #333; }}
                    .container {{ max-width: 600px; margin: 0 auto; padding: 20px; }}
                    .header {{ background-color: #2196F3; color: white; padding: 20px; text-align: center; }}
                    .content {{ padding: 20px; background-color: #f9f9f9; }}
                    .button {{ display: inline-block; padding: 12px 24px; background-color: #2196F3; color: white; text-decoration: none; border-radius: 4px; margin: 20px 0; }}
                    .warning {{ background-color: #fff3cd; border-left: 4px solid #ffc107; padding: 12px; margin: 20px 0; }}
                    .footer {{ text-align: center; padding: 20px; color: #666; font-size: 12px; }}
                </style>
            </head>
            <body>
                <div class="container">
                    <div class="header">
                        <h1>Password Reset Request</h1>
                    </div>
                    <div class="content">
                        <h2>Hello {}!</h2>
                        <p>We received a request to reset your password. Click the button below to create a new password:</p>
                        <a href="{}" class="button">Reset Password</a>
                        <p>Or copy and paste this link into your browser:</p>
                        <p style="word-break: break-all;">{}</p>
                        <div class="warning">
                            <strong>⚠️ Important:</strong> This link will expire in 1 hour for security reasons.
                        </div>
                        <p>If you didn't request a password reset, please ignore this email. Your password will remain unchanged.</p>
                    </div>
                    <div class="footer">
                        <p>&copy; 2025 Auth System. All rights reserved.</p>
                    </div>
                </div>
            </body>
            </html>
            "#,
            to_name, reset_url, reset_url
        );

        let text_body = format!(
            "Hello {}!\n\nWe received a request to reset your password. Visit this link to create a new password:\n{}\n\nThis link will expire in 1 hour.\n\nIf you didn't request a password reset, please ignore this email.",
            to_name, reset_url
        );

        let template = EmailTemplate {
            to_email: to_email.to_string(),
            to_name: Some(to_name.to_string()),
            subject: "Password Reset Request".to_string(),
            html_body,
            text_body,
        };

        self.send_email(template).await
    }

    /// Send welcome email after verification
    pub async fn send_welcome_email(
        &self,
        to_email: &str,
        to_name: &str,
    ) -> Result<(), AppError> {
        let html_body = format!(
            r#"
            <!DOCTYPE html>
            <html>
            <head>
                <style>
                    body {{ font-family: Arial, sans-serif; line-height: 1.6; color: #333; }}
                    .container {{ max-width: 600px; margin: 0 auto; padding: 20px; }}
                    .header {{ background-color: #4CAF50; color: white; padding: 20px; text-align: center; }}
                    .content {{ padding: 20px; background-color: #f9f9f9; }}
                    .button {{ display: inline-block; padding: 12px 24px; background-color: #4CAF50; color: white; text-decoration: none; border-radius: 4px; margin: 20px 0; }}
                    .footer {{ text-align: center; padding: 20px; color: #666; font-size: 12px; }}
                </style>
            </head>
            <body>
                <div class="container">
                    <div class="header">
                        <h1>Welcome! 🎉</h1>
                    </div>
                    <div class="content">
                        <h2>Hello {}!</h2>
                        <p>Your email has been successfully verified! Welcome to our community.</p>
                        <p>You can now enjoy all the features of our platform.</p>
                        <a href="{}" class="button">Get Started</a>
                        <p>If you have any questions, feel free to reach out to our support team.</p>
                    </div>
                    <div class="footer">
                        <p>&copy; 2025 Auth System. All rights reserved.</p>
                    </div>
                </div>
            </body>
            </html>
            "#,
            to_name, self.app_url
        );

        let text_body = format!(
            "Hello {}!\n\nYour email has been successfully verified! Welcome to our community.\n\nYou can now enjoy all the features of our platform.\n\nVisit: {}",
            to_name, self.app_url
        );

        let template = EmailTemplate {
            to_email: to_email.to_string(),
            to_name: Some(to_name.to_string()),
            subject: "Welcome! Your Account is Verified".to_string(),
            html_body,
            text_body,
        };

        self.send_email(template).await
    }

    /// Send password changed notification
    pub async fn send_password_changed_email(
        &self,
        to_email: &str,
        to_name: &str,
    ) -> Result<(), AppError> {
        let html_body = format!(
            r#"
            <!DOCTYPE html>
            <html>
            <head>
                <style>
                    body {{ font-family: Arial, sans-serif; line-height: 1.6; color: #333; }}
                    .container {{ max-width: 600px; margin: 0 auto; padding: 20px; }}
                    .header {{ background-color: #FF9800; color: white; padding: 20px; text-align: center; }}
                    .content {{ padding: 20px; background-color: #f9f9f9; }}
                    .warning {{ background-color: #fff3cd; border-left: 4px solid #ffc107; padding: 12px; margin: 20px 0; }}
                    .footer {{ text-align: center; padding: 20px; color: #666; font-size: 12px; }}
                </style>
            </head>
            <body>
                <div class="container">
                    <div class="header">
                        <h1>Password Changed</h1>
                    </div>
                    <div class="content">
                        <h2>Hello {}!</h2>
                        <p>This is a confirmation that your password has been changed successfully.</p>
                        <div class="warning">
                            <strong>⚠️ Security Notice:</strong> If you didn't make this change, please contact our support team immediately.
                        </div>
                        <p>For your security, all active sessions have been terminated and you'll need to login again with your new password.</p>
                    </div>
                    <div class="footer">
                        <p>&copy; 2025 Auth System. All rights reserved.</p>
                    </div>
                </div>
            </body>
            </html>
            "#,
            to_name
        );

        let text_body = format!(
            "Hello {}!\n\nThis is a confirmation that your password has been changed successfully.\n\nIf you didn't make this change, please contact our support team immediately.\n\nFor your security, all active sessions have been terminated.",
            to_name
        );

        let template = EmailTemplate {
            to_email: to_email.to_string(),
            to_name: Some(to_name.to_string()),
            subject: "Password Changed Successfully".to_string(),
            html_body,
            text_body,
        };

        self.send_email(template).await
    }

    /// Generic email sender with production SMTP support (public for queue usage)
    pub async fn send_email(&self, template: EmailTemplate) -> Result<(), AppError> {
        // Check if we're in development mode (no SMTP password set)
        let is_dev_mode = self.config.smtp_password.is_empty()
            || self.config.smtp_password == "your-smtp-password";

        if is_dev_mode {
            // Development mode: Just log the email
            tracing::info!(
                "📧 [DEV MODE] Email would be sent:\n\
                 To: {} ({})\n\
                 Subject: {}\n\
                 HTML Preview: {}...\n\
                 💡 Set SMTP_PASSWORD in .env to enable real email sending",
                template.to_email,
                template.to_name.as_deref().unwrap_or("Unknown"),
                template.subject,
                &template.html_body[..100.min(template.html_body.len())]
            );
            return Ok(());
        }

        // Production mode: Send actual email via SMTP
        use lettre::{
            message::{header::ContentType, Mailbox, MultiPart, SinglePart},
            transport::smtp::authentication::Credentials,
            Message, SmtpTransport, Transport,
        };

        // Build from mailbox
        let from_mailbox = format!("{} <{}>", self.config.from_name, self.config.from_email)
            .parse::<Mailbox>()
            .map_err(|e| AppError::Other(format!("Invalid from email: {}", e)))?;

        // Build to mailbox
        let to_mailbox = if let Some(name) = &template.to_name {
            format!("{} <{}>", name, template.to_email)
        } else {
            template.to_email.clone()
        }
        .parse::<Mailbox>()
        .map_err(|e| AppError::Other(format!("Invalid to email: {}", e)))?;

        // Build multipart email (HTML + plain text fallback)
        let email = Message::builder()
            .from(from_mailbox)
            .to(to_mailbox)
            .subject(&template.subject)
            .multipart(
                MultiPart::alternative()
                    .singlepart(
                        SinglePart::builder()
                            .header(ContentType::TEXT_PLAIN)
                            .body(template.text_body),
                    )
                    .singlepart(
                        SinglePart::builder()
                            .header(ContentType::TEXT_HTML)
                            .body(template.html_body),
                    ),
            )
            .map_err(|e| AppError::Other(format!("Failed to build email: {}", e)))?;

        // Create SMTP credentials
        let creds = Credentials::new(
            self.config.smtp_username.clone(),
            self.config.smtp_password.clone(),
        );

        // Build SMTP transport with SSL or STARTTLS based on configuration
        let mailer = if self.config.smtp_secure {
            // Use direct SSL connection (port 465)
            SmtpTransport::relay(&self.config.smtp_host)
                .map_err(|e| AppError::Other(format!("Failed to create SMTP transport: {}", e)))?
                .credentials(creds)
                .port(self.config.smtp_port)
                .build()
        } else {
            // Use STARTTLS (port 587)
            SmtpTransport::starttls_relay(&self.config.smtp_host)
                .map_err(|e| AppError::Other(format!("Failed to create SMTP transport: {}", e)))?
                .credentials(creds)
                .port(self.config.smtp_port)
                .build()
        };

        // Send the email
        mailer
            .send(&email)
            .map_err(|e| AppError::Other(format!("Failed to send email: {}", e)))?;

        tracing::info!(
            "✅ Email sent successfully to {} - Subject: '{}'",
            template.to_email,
            template.subject
        );

        Ok(())
    }
}

impl Default for EmailService {
    fn default() -> Self {
        Self::new()
    }
}
