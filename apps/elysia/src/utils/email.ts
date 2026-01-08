import nodemailer from 'nodemailer';

export interface EmailConfig {
  host: string;
  port: number;
  username: string;
  password: string;
  fromEmail: string;
  fromName: string;
}

export function getEmailConfig(): EmailConfig {
  return {
    host: process.env.SMTP_HOST || '',
    port: parseInt(process.env.SMTP_PORT || '0'),
    username: process.env.SMTP_USERNAME || '',
    password: process.env.SMTP_PASSWORD || '',
    fromEmail: process.env.FROM_EMAIL || 'noreply@example.com',
    fromName: process.env.FROM_NAME || 'App',
  };
}

export async function sendEmail(
  to: string,
  subject: string,
  text: string,
  html?: string,
): Promise<void> {
  const config = getEmailConfig();

  const transporter = nodemailer.createTransport({
    host: config.host,
    port: config.port,
    secure: config.port === 465,
    auth: {
      user: config.username,
      pass: config.password,
    },
  });

  await transporter.sendMail({
    from: `"${config.fromName}" <${config.fromEmail}>`,
    to,
    subject,
    text,
    html: html || text,
  });

  console.log(`📧 Email sent to ${to}`);
}

export async function sendVerificationEmail(
  email: string,
  username: string,
  token: string,
): Promise<void> {
  const verificationUrl = `${process.env.APP_URL || 'https://elysia.asepharyana.tech'}/api/auth/verify?token=${token}`;

  const subject = 'Verify Your Email Address';
  const text = `Hello ${username},\n\nPlease verify your email by clicking: ${verificationUrl}\n\nThis link expires in 24 hours.`;
  const html = `
    <h2>Welcome ${username}!</h2>
    <p>Please verify your email address by clicking the button below:</p>
    <a href="${verificationUrl}" style="display: inline-block; padding: 10px 20px; background-color: #007bff; color: white; text-decoration: none; border-radius: 5px;">
      Verify Email
    </a>
    <p>Or copy and paste this link: ${verificationUrl}</p>
    <p>This link expires in 24 hours.</p>
  `;

  await sendEmail(email, subject, text, html);
}

export async function sendPasswordResetEmail(
  email: string,
  username: string,
  token: string,
): Promise<void> {
  const resetUrl = `${process.env.APP_URL || 'https://elysia.asepharyana.tech'}/reset-password?token=${token}`;

  const subject = 'Reset Your Password';
  const text = `Hello ${username},\n\nReset your password by clicking: ${resetUrl}\n\nThis link expires in 1 hour.\nIf you didn't request this, please ignore this email.`;
  const html = `
    <h2>Password Reset Request</h2>
    <p>Hello ${username},</p>
    <p>Click the button below to reset your password:</p>
    <a href="${resetUrl}" style="display: inline-block; padding: 10px 20px; background-color: #dc3545; color: white; text-decoration: none; border-radius: 5px;">
      Reset Password
    </a>
    <p>Or copy and paste this link: ${resetUrl}</p>
    <p>This link expires in 1 hour.</p>
    <p>If you didn't request this, please ignore this email.</p>
  `;

  await sendEmail(email, subject, text, html);
}
