-- Create database if not exists
CREATE DATABASE IF NOT EXISTS `tracer_study`;

-- Create dedicated user for tracer_study
CREATE USER IF NOT EXISTS 'tracerstudy'@'%' IDENTIFIED BY 'tracerstudy_secret';

-- Grant privileges
GRANT ALL PRIVILEGES ON `tracer_study`.* TO 'tracerstudy'@'%';

-- Flush privileges
FLUSH PRIVILEGES;
