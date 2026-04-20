-- Create database if not exists
CREATE DATABASE IF NOT EXISTS `tracer_study`;

-- Create dedicated user for tracer_study
CREATE USER IF NOT EXISTS 'tracerstudy'@'%' IDENTIFIED BY 'tracerstudy_secret';
GRANT ALL PRIVILEGES ON `tracer_study`.* TO 'tracerstudy'@'%';

-- Create dedicated user for MySQL Exporter
CREATE USER IF NOT EXISTS 'exporter'@'%' IDENTIFIED BY 'exporterpassword';
GRANT PROCESS, REPLICATION CLIENT, SELECT ON *.* TO 'exporter'@'%';

-- Flush privileges
FLUSH PRIVILEGES;
