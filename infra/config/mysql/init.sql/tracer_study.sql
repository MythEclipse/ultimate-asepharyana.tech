-- phpMyAdmin SQL Dump
-- version 5.2.0
-- https://www.phpmyadmin.net/
--
-- Host: localhost:3306
-- Generation Time: Mar 07, 2026 at 10:39 AM
-- Server version: 8.0.30
-- PHP Version: 8.3.19

SET SQL_MODE = "NO_AUTO_VALUE_ON_ZERO";
START TRANSACTION;
SET time_zone = "+00:00";


/*!40101 SET @OLD_CHARACTER_SET_CLIENT=@@CHARACTER_SET_CLIENT */;
/*!40101 SET @OLD_CHARACTER_SET_RESULTS=@@CHARACTER_SET_RESULTS */;
/*!40101 SET @OLD_COLLATION_CONNECTION=@@COLLATION_CONNECTION */;
/*!40101 SET NAMES utf8mb4 */;

--
-- Database: `tracer_study`
--

-- --------------------------------------------------------

--
-- Table structure for table `alumnis`
--

CREATE TABLE `alumnis` (
  `id` bigint UNSIGNED NOT NULL,
  `user_id` bigint UNSIGNED NOT NULL,
  `nim` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `username` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `birth_place` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `birth_date` date NOT NULL,
  `gender` enum('male','female') COLLATE utf8mb4_unicode_ci NOT NULL,
  `address` text COLLATE utf8mb4_unicode_ci NOT NULL,
  `phone_number` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `email` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `gpa` decimal(3,2) NOT NULL,
  `faculty` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `major` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `entry_year` year NOT NULL,
  `graduation_year` year NOT NULL,
  `status` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `category_field` varchar(255) COLLATE utf8mb4_unicode_ci DEFAULT NULL,
  `waiting_period` int DEFAULT NULL,
  `job_position` varchar(255) COLLATE utf8mb4_unicode_ci DEFAULT NULL,
  `company_name` varchar(255) COLLATE utf8mb4_unicode_ci DEFAULT NULL,
  `foto` varchar(255) COLLATE utf8mb4_unicode_ci DEFAULT NULL,
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

--
-- Dumping data for table `alumnis`
--

INSERT INTO `alumnis` (`id`, `user_id`, `nim`, `username`, `birth_place`, `birth_date`, `gender`, `address`, `phone_number`, `email`, `gpa`, `faculty`, `major`, `entry_year`, `graduation_year`, `status`, `category_field`, `waiting_period`, `job_position`, `company_name`, `foto`, `created_at`, `updated_at`) VALUES
(11, 12, '701220220', 'Jaka', 'Jambi', '2003-12-18', 'male', 'muaro,jambi', '081520222070', 'jaka@gmail.com', '4.00', 'Sains dan Teknologi', 'Sistem Informasi', 2023, 2027, 'bekerja', 'product & analysis', 2, 'Business Analyst', 'PT Teknologi Maju', NULL, '2025-12-28 13:22:37', '2026-01-21 00:23:19'),
(12, 13, '701220127', 'Nadia Oktarina', 'Palembang', '2003-10-01', 'female', 'muaro,jambi', '081520222026', 'nadiaokta847@gmail.com', '4.00', 'Sains dan Teknologi', 'Sistem Informasi', 2022, 2026, 'bekerja', 'design & ux', 1, 'UI/UX Designer', 'PT PERTAMINA PERSERO', NULL, '2025-12-28 13:53:07', '2026-03-03 03:24:50'),
(13, 14, '701220001', 'Ahmad Fauzi', 'Bandung', '2000-01-15', 'male', 'Jl. Merdeka No 1', '081234567801', 'ahmad.fauzi@gmail.com', '3.45', 'Sains dan Teknologi', 'Sistem Informasi', 2018, 2022, 'bekerja', 'it & development', 7, 'Programmer', 'PT Teknologi Maju', NULL, '2025-12-28 14:25:04', '2026-01-21 00:34:57'),
(14, 15, '701220002', 'Siti Aisyah', 'Jakarta', '1999-05-20', 'female', 'Jl. Sudirman No 10', '081234567802', 'siti.aisyah@gmail.com', '3.80', 'Sains dan Teknologi', 'Sistem Informasi', 2017, 2021, 'bekerja', 'product & analysis', 9, 'System Analyst', 'PT Digital Solusi', NULL, '2025-12-28 14:25:04', '2026-01-21 00:34:32'),
(15, 16, '701220003', 'Budi Santoso', 'Surabaya', '2000-03-10', 'male', 'Jl. Pahlawan No 5', '081234567803', 'budi.santoso@gmail.com', '3.20', 'Sains dan Teknologi', 'Sistem Informasi', 2018, 2022, 'belum bekerja', NULL, NULL, NULL, NULL, NULL, '2025-12-28 14:25:05', '2026-01-21 00:33:56'),
(16, 17, '701220004', 'Rina Marlina', 'Bogor', '2001-07-08', 'female', 'Jl. Raya Bogor Km 30', '081234567804', 'rina.marlina@gmail.com', '3.60', 'Sains dan Teknologi', 'Sistem Informasi', 2019, 2023, 'bekerja', 'design & ux', 19, 'UI/UX Designer', 'PT Kreatif Digital', NULL, '2025-12-28 14:25:05', '2026-01-21 00:33:26'),
(17, 18, '701220005', 'Dedi Pratama', 'Medan', '1999-11-25', 'male', 'Jl. Gatot Subroto No 12', '081234567805', 'dedi.pratama@gmail.com', '3.10', 'Sains dan Teknologi', 'Sistem Informasi', 2017, 2021, 'belum bekerja', NULL, NULL, NULL, NULL, NULL, '2025-12-28 14:25:05', '2026-01-21 00:32:45'),
(18, 19, '701220006', 'Lina Putri', 'Padang', '2000-09-14', 'female', 'Jl. Veteran No 7', '081234567806', 'lina.putri@gmail.com', '3.75', 'Sains dan Teknologi', 'Sistem Informasi', 2018, 2022, 'bekerja', 'product & analysis', 24, 'Data Analyst', 'PT Data Cerdas', NULL, '2025-12-28 14:25:06', '2026-01-21 00:32:19'),
(19, 20, '701220007', 'Rizky Maulana', 'Depok', '2001-02-02', 'male', 'Jl. Margonda Raya', '081234567807', 'rizky.maulana@gmail.com', '3.50', 'Sains dan Teknologi', 'Sistem Informasi', 2019, 2023, 'bekerja', 'it & development', 30, 'Backend Developer', 'PT Software Nusantara', NULL, '2025-12-28 14:25:06', '2026-01-21 00:31:07'),
(20, 21, '701220008', 'Putri Ananda', 'Palembang', '2000-06-18', 'female', 'Jl. Sudirman Ilir', '081234567808', 'putri.ananda@gmail.com', '3.90', 'Sains dan Teknologi', 'Sistem Informasi', 2018, 2022, 'bekerja', 'it & development', 22, 'Mobile Developer', 'PT Aplikasi Hebat', NULL, '2025-12-28 14:25:07', '2026-01-21 00:30:35'),
(21, 22, '701220009', 'Andi Wijaya', 'Makassar', '1999-12-30', 'male', 'Jl. Pettarani No 9', '081234567809', 'andi.wijaya@gmail.com', '3.00', 'Sains dan Teknologi', 'Sistem Informasi', 2017, 2021, 'belum bekerja', NULL, NULL, NULL, NULL, NULL, '2025-12-28 14:25:07', '2026-01-21 00:29:08'),
(22, 23, '701220010', 'Nurul Hidayah', 'Yogyakarta', '2001-04-05', 'female', 'Jl. Kaliurang Km 5', '081234567810', 'nurul.hidayah@gmail.com', '3.85', 'Sains dan Teknologi', 'Sistem Informasi', 2019, 2023, 'bekerja', 'product & analysis', 19, 'Risk Analyst', 'PT BTN tbk', NULL, '2025-12-28 14:25:07', '2026-01-21 00:29:43'),
(80, 50, '701220015', 'Dedi Pratama', 'Medan', '1996-03-23', 'male', 'Jl. Gatot Subroto', '081355566678', 'dedi.pratama20@gmail.com', '3.10', 'Sains dan Teknologi', 'Sistem Informasi', 2016, 2020, 'wirausaha', NULL, NULL, NULL, 'Usaha Mandiri', NULL, '2026-01-13 02:44:37', '2026-01-21 00:28:44'),
(98, 68, '701220011', 'Ahmad Fauzi', 'Bandung', '1999-05-12', 'male', 'Jl. Merdeka No. 10', '081234567890', 'ahmad.fauzi02@gmail.com', '3.45', 'Sains dan Teknologi', 'Sistem Informasi', 2018, 2022, 'bekerja', 'it & development', 2, 'Software Engineer', 'PT Teknologi Nusantara', NULL, '2026-01-13 03:06:32', '2026-01-21 00:28:04'),
(99, 69, '701220012', 'Siti Aminah', 'Jakarta', '2000-02-20', 'female', 'Jl. Sudirman No. 5', '081298765432', 'siti.aminah@gmail.com', '3.67', 'Sains dan Teknologi', 'Sistem Informasi', 2019, 2023, 'bekerja', 'product & analysis', 3, 'Product Analyst', 'PT Digital Solusi', NULL, '2026-01-13 03:06:32', '2026-01-21 00:27:30'),
(100, 70, '701220013', 'Budi Santoso', 'Surabaya', '1998-11-03', 'male', 'Jl. Ahmad Yani No. 21', '081377788899', 'budi.santoso22@gmail.com', '3.20', 'Sains dan Teknologi', 'Sistem Informasi', 2017, 2021, 'belum bekerja', NULL, NULL, NULL, NULL, NULL, '2026-01-13 03:06:33', '2026-01-21 00:26:52'),
(101, 84, '701220014', 'Rina badria', 'Yogyakarta', '1999-07-15', 'female', 'Jl. Kaliurang Km 7', '081366679088', 'rinabadr@gmail.com', '3.80', 'Sains dan Teknologi', 'Sistem Informasi', 2019, 2023, 'bekerja', 'business & management', 1, 'Business Analyst', 'PT Konsultan Maju', NULL, '2026-01-13 03:06:34', '2026-02-27 21:36:08'),
(102, 72, '701220133', 'M Syarifuddin', 'Palembang', '2003-01-16', 'male', 'jambi, batang hari', '081520222234', 'udin@gmail.com', '3.29', 'Sains dan Teknologi', 'Sistem Informasi', 2020, 2024, 'wirausaha', NULL, NULL, NULL, 'Bisnis Service Leptop', NULL, '2026-01-14 08:09:07', '2026-01-21 00:25:24'),
(104, 78, '701220251', 'Ahmad Fairuz Akbar', 'Surakarta', '2000-02-27', 'male', 'Jl. Merdeka No. 10', '81234567890', 'ahmadfairuz1@gmail.com', '3.45', 'Fakultas Ilmu Komputer', 'Sistem Informasi', 2018, 2022, 'bekerja', 'it & development', 2, 'Software Engineer', 'PT Teknologi Nusantara', NULL, '2026-01-26 01:00:24', '2026-02-28 00:50:13'),
(106, 80, '701220335', 'Mustofa', 'Palembang', '2002-10-12', 'male', 'Jln Sunan Ampel No 10, Kota Palembang', '081532236770', 'mustop4@gmail.com', '3.98', 'Sains dan Teknologi', 'Sistem Informasi', 2021, 2025, 'belum bekerja', NULL, NULL, NULL, NULL, NULL, '2026-02-27 20:13:10', '2026-02-27 20:13:10'),
(107, 81, '701220515', 'Ikbal', 'Bandung', '1999-08-12', 'male', 'Jl. Merdeka No. 10', '081234556890', 'ikbal2@gmail.com', '3.65', 'Sains dan Teknologi', 'Sistem Informasi', 2019, 2023, 'bekerja', 'it & development', 6, 'Software Engineer', 'PT Teknologi Nusantara', NULL, '2026-02-27 21:22:01', '2026-02-27 21:22:01'),
(108, 82, '701220516', 'Siti Rohaya', 'Brebes', '2001-02-20', 'female', 'Jl. Sudirman No. 5', '081599765432', 'sitiryh@gmail.com', '3.69', 'Sains dan Teknologi', 'Sistem Informasi', 2019, 2023, 'bekerja', 'product & analysis', 10, 'Product Analyst', 'PT Digital Solusi', NULL, '2026-02-27 21:22:01', '2026-02-27 21:22:01'),
(109, 83, '701220518', 'Budiman', 'Jakarta', '1999-12-03', 'male', 'Jl. Ahmad Yani No. 21', '081377788899', 'budiman02@gmail.com', '3.70', 'Fakultas Ilmu Komputer', 'Sistem Informasi', 2019, 2023, 'belum bekerja', NULL, NULL, NULL, NULL, NULL, '2026-02-27 21:22:02', '2026-02-27 21:36:07'),
(123, 85, '701220615', 'Rendi Pratama', 'Medan', '1999-03-08', 'male', 'Jl. Gatot Subroto', '081353566677', 'rendipratama10@gmail.com', '3.70', 'Sains dan Teknologi', 'Sistem Informasi', 2019, 2023, 'bekerja', 'operation & support', 4, 'IT Support', 'Usaha Mandiri', NULL, '2026-02-27 21:36:09', '2026-02-27 21:36:09');

-- --------------------------------------------------------

--
-- Table structure for table `cache`
--

CREATE TABLE `cache` (
  `key` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `value` mediumtext COLLATE utf8mb4_unicode_ci NOT NULL,
  `expiration` int NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

--
-- Dumping data for table `cache`
--

INSERT INTO `cache` (`key`, `value`, `expiration`) VALUES
('tracer-study-cache-livewire-rate-limiter:62dcdddd2131784090e8a5916c260a90678e71b3', 'i:1;', 1772534892),
('tracer-study-cache-livewire-rate-limiter:62dcdddd2131784090e8a5916c260a90678e71b3:timer', 'i:1772534892;', 1772534892),
('tracer-study-cache-livewire-rate-limiter:a17961fa74e9275d529f489537f179c05d50c2f3', 'i:1;', 1772537303),
('tracer-study-cache-livewire-rate-limiter:a17961fa74e9275d529f489537f179c05d50c2f3:timer', 'i:1772537303;', 1772537303),
('tracer-study-cache-livewire-rate-limiter:c9648cc0cb3d6646e566f360255b7283d9012d02', 'i:2;', 1772535874),
('tracer-study-cache-livewire-rate-limiter:c9648cc0cb3d6646e566f360255b7283d9012d02:timer', 'i:1772535874;', 1772535874),
('tracer-study-cache-spatie.permission.cache', 'a:3:{s:5:\"alias\";a:4:{s:1:\"a\";s:2:\"id\";s:1:\"b\";s:4:\"name\";s:1:\"c\";s:10:\"guard_name\";s:1:\"r\";s:5:\"roles\";}s:11:\"permissions\";a:33:{i:0;a:4:{s:1:\"a\";i:1;s:1:\"b\";s:11:\"view_alumni\";s:1:\"c\";s:3:\"web\";s:1:\"r\";a:1:{i:0;i:1;}}i:1;a:4:{s:1:\"a\";i:2;s:1:\"b\";s:13:\"create_alumni\";s:1:\"c\";s:3:\"web\";s:1:\"r\";a:1:{i:0;i:1;}}i:2;a:4:{s:1:\"a\";i:3;s:1:\"b\";s:11:\"edit_alumni\";s:1:\"c\";s:3:\"web\";s:1:\"r\";a:1:{i:0;i:1;}}i:3;a:4:{s:1:\"a\";i:4;s:1:\"b\";s:13:\"delete_alumni\";s:1:\"c\";s:3:\"web\";s:1:\"r\";a:1:{i:0;i:1;}}i:4;a:4:{s:1:\"a\";i:5;s:1:\"b\";s:13:\"import_alumni\";s:1:\"c\";s:3:\"web\";s:1:\"r\";a:1:{i:0;i:1;}}i:5;a:4:{s:1:\"a\";i:6;s:1:\"b\";s:13:\"export_alumni\";s:1:\"c\";s:3:\"web\";s:1:\"r\";a:1:{i:0;i:1;}}i:6;a:4:{s:1:\"a\";i:7;s:1:\"b\";s:18:\"view_questionnaire\";s:1:\"c\";s:3:\"web\";s:1:\"r\";a:1:{i:0;i:1;}}i:7;a:4:{s:1:\"a\";i:8;s:1:\"b\";s:20:\"create_questionnaire\";s:1:\"c\";s:3:\"web\";s:1:\"r\";a:1:{i:0;i:1;}}i:8;a:4:{s:1:\"a\";i:9;s:1:\"b\";s:18:\"edit_questionnaire\";s:1:\"c\";s:3:\"web\";s:1:\"r\";a:1:{i:0;i:1;}}i:9;a:4:{s:1:\"a\";i:10;s:1:\"b\";s:20:\"delete_questionnaire\";s:1:\"c\";s:3:\"web\";s:1:\"r\";a:1:{i:0;i:1;}}i:10;a:4:{s:1:\"a\";i:11;s:1:\"b\";s:14:\"view_responses\";s:1:\"c\";s:3:\"web\";s:1:\"r\";a:1:{i:0;i:1;}}i:11;a:4:{s:1:\"a\";i:12;s:1:\"b\";s:16:\"export_responses\";s:1:\"c\";s:3:\"web\";s:1:\"r\";a:1:{i:0;i:1;}}i:12;a:4:{s:1:\"a\";i:13;s:1:\"b\";s:18:\"fill_questionnaire\";s:1:\"c\";s:3:\"web\";s:1:\"r\";a:1:{i:0;i:2;}}i:13;a:4:{s:1:\"a\";i:14;s:1:\"b\";s:8:\"view_job\";s:1:\"c\";s:3:\"web\";s:1:\"r\";a:2:{i:0;i:1;i:1;i:2;}}i:14;a:4:{s:1:\"a\";i:15;s:1:\"b\";s:10:\"create_job\";s:1:\"c\";s:3:\"web\";s:1:\"r\";a:2:{i:0;i:1;i:1;i:2;}}i:15;a:4:{s:1:\"a\";i:16;s:1:\"b\";s:12:\"edit_own_job\";s:1:\"c\";s:3:\"web\";s:1:\"r\";a:2:{i:0;i:1;i:1;i:2;}}i:16;a:4:{s:1:\"a\";i:17;s:1:\"b\";s:14:\"delete_own_job\";s:1:\"c\";s:3:\"web\";s:1:\"r\";a:2:{i:0;i:1;i:1;i:2;}}i:17;a:4:{s:1:\"a\";i:18;s:1:\"b\";s:16:\"view_scholarship\";s:1:\"c\";s:3:\"web\";s:1:\"r\";a:2:{i:0;i:1;i:1;i:2;}}i:18;a:4:{s:1:\"a\";i:19;s:1:\"b\";s:18:\"create_scholarship\";s:1:\"c\";s:3:\"web\";s:1:\"r\";a:2:{i:0;i:1;i:1;i:2;}}i:19;a:4:{s:1:\"a\";i:20;s:1:\"b\";s:20:\"edit_own_scholarship\";s:1:\"c\";s:3:\"web\";s:1:\"r\";a:2:{i:0;i:1;i:1;i:2;}}i:20;a:4:{s:1:\"a\";i:21;s:1:\"b\";s:22:\"delete_own_scholarship\";s:1:\"c\";s:3:\"web\";s:1:\"r\";a:2:{i:0;i:1;i:1;i:2;}}i:21;a:4:{s:1:\"a\";i:22;s:1:\"b\";s:15:\"view_internship\";s:1:\"c\";s:3:\"web\";s:1:\"r\";a:2:{i:0;i:1;i:1;i:2;}}i:22;a:4:{s:1:\"a\";i:23;s:1:\"b\";s:17:\"create_internship\";s:1:\"c\";s:3:\"web\";s:1:\"r\";a:2:{i:0;i:1;i:1;i:2;}}i:23;a:4:{s:1:\"a\";i:24;s:1:\"b\";s:19:\"edit_own_internship\";s:1:\"c\";s:3:\"web\";s:1:\"r\";a:2:{i:0;i:1;i:1;i:2;}}i:24;a:4:{s:1:\"a\";i:25;s:1:\"b\";s:21:\"delete_own_internship\";s:1:\"c\";s:3:\"web\";s:1:\"r\";a:2:{i:0;i:1;i:1;i:2;}}i:25;a:4:{s:1:\"a\";i:26;s:1:\"b\";s:20:\"view_admin_dashboard\";s:1:\"c\";s:3:\"web\";s:1:\"r\";a:1:{i:0;i:1;}}i:26;a:4:{s:1:\"a\";i:27;s:1:\"b\";s:21:\"view_alumni_dashboard\";s:1:\"c\";s:3:\"web\";s:1:\"r\";a:1:{i:0;i:2;}}i:27;a:4:{s:1:\"a\";i:28;s:1:\"b\";s:10:\"view_forum\";s:1:\"c\";s:3:\"web\";s:1:\"r\";a:2:{i:0;i:1;i:1;i:2;}}i:28;a:4:{s:1:\"a\";i:29;s:1:\"b\";s:18:\"create_forum_topic\";s:1:\"c\";s:3:\"web\";s:1:\"r\";a:2:{i:0;i:1;i:1;i:2;}}i:29;a:4:{s:1:\"a\";i:30;s:1:\"b\";s:20:\"edit_own_forum_topic\";s:1:\"c\";s:3:\"web\";s:1:\"r\";a:2:{i:0;i:1;i:1;i:2;}}i:30;a:4:{s:1:\"a\";i:31;s:1:\"b\";s:22:\"delete_own_forum_topic\";s:1:\"c\";s:3:\"web\";s:1:\"r\";a:2:{i:0;i:1;i:1;i:2;}}i:31;a:4:{s:1:\"a\";i:32;s:1:\"b\";s:17:\"reply_forum_topic\";s:1:\"c\";s:3:\"web\";s:1:\"r\";a:2:{i:0;i:1;i:1;i:2;}}i:32;a:4:{s:1:\"a\";i:33;s:1:\"b\";s:14:\"moderate_forum\";s:1:\"c\";s:3:\"web\";s:1:\"r\";a:1:{i:0;i:1;}}}s:5:\"roles\";a:2:{i:0;a:3:{s:1:\"a\";i:1;s:1:\"b\";s:5:\"admin\";s:1:\"c\";s:3:\"web\";}i:1;a:3:{s:1:\"a\";i:2;s:1:\"b\";s:6:\"alumni\";s:1:\"c\";s:3:\"web\";}}}', 1772622265);

-- --------------------------------------------------------

--
-- Table structure for table `cache_locks`
--

CREATE TABLE `cache_locks` (
  `key` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `owner` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `expiration` int NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

--
-- Dumping data for table `cache_locks`
--

INSERT INTO `cache_locks` (`key`, `owner`, `expiration`) VALUES
('tracer-study-cache-laravel-queue-overlap:Filament\\Actions\\Imports\\Jobs\\ImportCsv:import3', 'rPXYSGIT8gg4QdZR', 1768297580),
('tracer-study-cache-laravel-queue-overlap:Filament\\Actions\\Imports\\Jobs\\ImportCsv:import4', 'tDnwFiMR7ZsnggVu', 1768298088),
('tracer-study-cache-laravel-queue-overlap:Filament\\Actions\\Imports\\Jobs\\ImportCsv:import6', 'a1iKRUkkRI4MSMv8', 1772253156);

-- --------------------------------------------------------

--
-- Table structure for table `exports`
--

CREATE TABLE `exports` (
  `id` bigint UNSIGNED NOT NULL,
  `completed_at` timestamp NULL DEFAULT NULL,
  `file_disk` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `file_name` varchar(255) COLLATE utf8mb4_unicode_ci DEFAULT NULL,
  `exporter` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `processed_rows` int UNSIGNED NOT NULL DEFAULT '0',
  `total_rows` int UNSIGNED NOT NULL,
  `successful_rows` int UNSIGNED NOT NULL DEFAULT '0',
  `user_id` bigint UNSIGNED NOT NULL,
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

--
-- Dumping data for table `exports`
--

INSERT INTO `exports` (`id`, `completed_at`, `file_disk`, `file_name`, `exporter`, `processed_rows`, `total_rows`, `successful_rows`, `user_id`, `created_at`, `updated_at`) VALUES
(1, '2025-12-28 12:51:02', 'local', 'export-1-alumnis', 'App\\Filament\\Exports\\AlumniExporter', 10, 10, 10, 1, '2025-12-28 12:51:01', '2025-12-28 12:51:02'),
(2, '2025-12-29 21:16:11', 'local', 'export-2-questionnaire-responses', 'App\\Filament\\Exports\\QuestionnaireResponseExporter', 2, 2, 2, 1, '2025-12-29 21:16:05', '2025-12-29 21:16:11'),
(3, '2026-01-13 02:32:40', 'local', 'export-3-alumnis', 'App\\Filament\\Exports\\AlumniExporter', 13, 13, 13, 1, '2026-01-13 02:32:33', '2026-01-13 02:32:40'),
(4, '2026-01-27 02:07:27', 'local', 'export-4-questionnaire-responses', 'App\\Filament\\Exports\\QuestionnaireResponseExporter', 10, 10, 10, 1, '2026-01-27 02:07:22', '2026-01-27 02:07:27'),
(5, '2026-01-29 03:11:14', 'local', 'export-5-questionnaire-responses', 'App\\Filament\\Exports\\QuestionnaireResponseExporter', 7, 7, 7, 1, '2026-01-29 03:11:05', '2026-01-29 03:11:14'),
(6, '2026-02-01 05:22:46', 'local', 'ekspor-6-questionnaire-responses', 'App\\Filament\\Exports\\QuestionnaireResponseExporter', 1, 1, 1, 1, '2026-02-01 05:22:39', '2026-02-01 05:22:46');

-- --------------------------------------------------------

--
-- Table structure for table `failed_import_rows`
--

CREATE TABLE `failed_import_rows` (
  `id` bigint UNSIGNED NOT NULL,
  `data` json NOT NULL,
  `import_id` bigint UNSIGNED NOT NULL,
  `validation_error` text COLLATE utf8mb4_unicode_ci,
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- --------------------------------------------------------

--
-- Table structure for table `failed_jobs`
--

CREATE TABLE `failed_jobs` (
  `id` bigint UNSIGNED NOT NULL,
  `uuid` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `connection` text COLLATE utf8mb4_unicode_ci NOT NULL,
  `queue` text COLLATE utf8mb4_unicode_ci NOT NULL,
  `payload` longtext COLLATE utf8mb4_unicode_ci NOT NULL,
  `exception` longtext COLLATE utf8mb4_unicode_ci NOT NULL,
  `failed_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- --------------------------------------------------------

--
-- Table structure for table `forum_replies`
--

CREATE TABLE `forum_replies` (
  `id` bigint UNSIGNED NOT NULL,
  `topic_id` bigint UNSIGNED NOT NULL,
  `user_id` bigint UNSIGNED NOT NULL,
  `content` text COLLATE utf8mb4_unicode_ci NOT NULL,
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

--
-- Dumping data for table `forum_replies`
--

INSERT INTO `forum_replies` (`id`, `topic_id`, `user_id`, `content`, `created_at`, `updated_at`) VALUES
(1, 1, 78, 'info dong', '2026-01-31 05:54:19', '2026-01-31 05:54:19'),
(11, 3, 78, 'baik pak', '2026-02-01 03:27:51', '2026-02-01 03:27:51');

-- --------------------------------------------------------

--
-- Table structure for table `forum_topics`
--

CREATE TABLE `forum_topics` (
  `id` bigint UNSIGNED NOT NULL,
  `user_id` bigint UNSIGNED NOT NULL,
  `title` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `content` text COLLATE utf8mb4_unicode_ci NOT NULL,
  `is_pinned` tinyint(1) NOT NULL DEFAULT '0',
  `is_locked` tinyint(1) NOT NULL DEFAULT '0',
  `views_count` int NOT NULL DEFAULT '0',
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

--
-- Dumping data for table `forum_topics`
--

INSERT INTO `forum_topics` (`id`, `user_id`, `title`, `content`, `is_pinned`, `is_locked`, `views_count`, `created_at`, `updated_at`) VALUES
(1, 78, 'Info Magang', '<p><strong>teman-teman adakah info magang sebagai admin disekitaran mendalo</strong></p>', 0, 1, 66, '2026-01-31 05:46:15', '2026-02-28 01:12:31'),
(2, 1, 'Isi kuesioner ', '<p>S<strong>ilahkan isi kuesioner tahunan kalian</strong></p>', 0, 0, 42, '2026-01-31 06:11:51', '2026-02-28 01:12:58'),
(3, 1, 'Info Pengambilan Ijazah', '<p><strong>Yang posisi lagi di jambi, kalo mau ambil ijazah tunggu masuk kuliah tanggal 22</strong></p><p><span style=\"text-decoration: underline;\">note: bawa kelengkapan berkas</span></p>', 1, 0, 10, '2026-02-01 03:22:35', '2026-02-28 01:12:13');

-- --------------------------------------------------------

--
-- Table structure for table `imports`
--

CREATE TABLE `imports` (
  `id` bigint UNSIGNED NOT NULL,
  `completed_at` timestamp NULL DEFAULT NULL,
  `file_name` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `file_path` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `importer` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `processed_rows` int UNSIGNED NOT NULL DEFAULT '0',
  `total_rows` int UNSIGNED NOT NULL,
  `successful_rows` int UNSIGNED NOT NULL DEFAULT '0',
  `user_id` bigint UNSIGNED NOT NULL,
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

--
-- Dumping data for table `imports`
--

INSERT INTO `imports` (`id`, `completed_at`, `file_name`, `file_path`, `importer`, `processed_rows`, `total_rows`, `successful_rows`, `user_id`, `created_at`, `updated_at`) VALUES
(1, '2025-12-28 14:25:08', 'alumni_import.csv', 'D:\\laragon\\www\\sisfor_alumni2\\filament-starter\\storage\\app/private\\livewire-tmp/cWfV2xhlYy2Rxr9PHGGeXoHjYZ0VLH-metaYWx1bW5pX2ltcG9ydC5jc3Y=-.csv', 'App\\Filament\\Imports\\AlumniImporter', 10, 10, 10, 1, '2025-12-28 14:25:02', '2025-12-28 14:25:08'),
(2, '2026-01-02 22:24:17', 'alumni_import.csv', 'D:\\laragon\\www\\sisfor_alumni2\\filament-starter\\storage\\app/private\\livewire-tmp/Qw3Se9MhdU3K7qQjEknAkuRtbZPM75-metaYWx1bW5pX2ltcG9ydC5jc3Y=-.csv', 'App\\Filament\\Imports\\AlumniImporter', 10, 10, 10, 1, '2026-01-02 22:24:08', '2026-01-02 22:24:17'),
(5, '2026-01-13 03:06:34', 'template_import_alumni.csv', 'D:\\laragon\\www\\sisfor_alumni2\\filament-starter\\storage\\app/private\\livewire-tmp/q03r9qdRsjBSWJSeLmTidKfSoxeXSc-metadGVtcGxhdGVfaW1wb3J0X2FsdW1uaS5jc3Y=-.csv', 'App\\Filament\\Imports\\AlumniImporter', 5, 5, 5, 1, '2026-01-13 03:06:29', '2026-01-13 03:06:34'),
(7, '2026-02-27 21:36:09', 'test1.csv', 'D:\\laragon\\www\\sisfor_alumni2\\filament-starter\\storage\\app/private\\livewire-tmp/a6mIDStEO7yGVnbdW4Nq5PGkARAMZL-metadGVzdDEuY3N2-.csv', 'App\\Filament\\Imports\\AlumniImporter', 5, 5, 5, 1, '2026-02-27 21:36:02', '2026-02-27 21:36:09');

-- --------------------------------------------------------

--
-- Table structure for table `internships`
--

CREATE TABLE `internships` (
  `id` bigint UNSIGNED NOT NULL,
  `user_id` bigint UNSIGNED NOT NULL,
  `title` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `company` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `location` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `description` text COLLATE utf8mb4_unicode_ci NOT NULL,
  `requirements` text COLLATE utf8mb4_unicode_ci,
  `duration` varchar(255) COLLATE utf8mb4_unicode_ci DEFAULT NULL,
  `deadline` date DEFAULT NULL,
  `contact_info` varchar(255) COLLATE utf8mb4_unicode_ci DEFAULT NULL,
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

--
-- Dumping data for table `internships`
--

INSERT INTO `internships` (`id`, `user_id`, `title`, `company`, `location`, `description`, `requirements`, `duration`, `deadline`, `contact_info`, `created_at`, `updated_at`) VALUES
(1, 18, 'Magang Web Developer', 'PT Teknologi Maju Bersama', 'Yogyakarta, DI Yogyakarta', '<p>&nbsp;PT Teknologi Maju Bersama membuka program magang Web Developer bagi fresh graduate yang ingin mengembangkan kemampuan dalam pengembangan aplikasi berbasis web. Peserta magang akan terlibat langsung dalam proyek pengembangan internal perusahaan.&nbsp;</p>', '<ul><li>Mahasiswa aktif atau fresh graduate jurusan Sistem Informasi / Teknik Informatika</li><li>Menguasai dasar HTML, CSS, dan JavaScript</li><li>Memahami framework Laravel menjadi nilai tambah</li><li>Bersedia bekerja secara tim dan mengikuti jadwal magang</li></ul>', '6 bulan', '2026-02-05', 'internship@teknologimaju.co.id', '2026-01-02 23:13:14', '2026-01-02 23:13:14');

-- --------------------------------------------------------

--
-- Table structure for table `jobs`
--

CREATE TABLE `jobs` (
  `id` bigint UNSIGNED NOT NULL,
  `queue` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `payload` longtext COLLATE utf8mb4_unicode_ci NOT NULL,
  `attempts` tinyint UNSIGNED NOT NULL,
  `reserved_at` int UNSIGNED DEFAULT NULL,
  `available_at` int UNSIGNED NOT NULL,
  `created_at` int UNSIGNED NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- --------------------------------------------------------

--
-- Table structure for table `job_batches`
--

CREATE TABLE `job_batches` (
  `id` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `name` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `total_jobs` int NOT NULL,
  `pending_jobs` int NOT NULL,
  `failed_jobs` int NOT NULL,
  `failed_job_ids` longtext COLLATE utf8mb4_unicode_ci NOT NULL,
  `options` mediumtext COLLATE utf8mb4_unicode_ci,
  `cancelled_at` int DEFAULT NULL,
  `created_at` int NOT NULL,
  `finished_at` int DEFAULT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

--
-- Dumping data for table `job_batches`
--

INSERT INTO `job_batches` (`id`, `name`, `total_jobs`, `pending_jobs`, `failed_jobs`, `failed_job_ids`, `options`, `cancelled_at`, `created_at`, `finished_at`) VALUES
('a0b40b37-4a90-4670-a51d-f6fd8076b373', '', 1, 0, 0, '[]', 'a:2:{s:13:\"allowFailures\";b:1;s:7:\"finally\";a:1:{i:0;O:47:\"Laravel\\SerializableClosure\\SerializableClosure\":1:{s:12:\"serializable\";O:46:\"Laravel\\SerializableClosure\\Serializers\\Signed\":2:{s:12:\"serializable\";s:3952:\"O:46:\"Laravel\\SerializableClosure\\Serializers\\Native\":5:{s:3:\"use\";a:4:{s:9:\"columnMap\";a:17:{s:3:\"nim\";s:3:\"nim\";s:8:\"username\";s:8:\"username\";s:11:\"birth_place\";s:11:\"birth_place\";s:10:\"birth_date\";s:10:\"birth_date\";s:6:\"gender\";s:6:\"gender\";s:7:\"address\";s:7:\"address\";s:12:\"phone_number\";s:12:\"phone_number\";s:5:\"email\";s:5:\"email\";s:3:\"gpa\";s:3:\"gpa\";s:7:\"faculty\";s:7:\"faculty\";s:5:\"major\";s:5:\"major\";s:10:\"entry_year\";s:10:\"entry_year\";s:15:\"graduation_year\";s:15:\"graduation_year\";s:6:\"status\";s:6:\"status\";s:12:\"job_position\";s:12:\"job_position\";s:12:\"company_name\";s:12:\"company_name\";s:4:\"foto\";s:4:\"foto\";}s:6:\"import\";O:45:\"Illuminate\\Contracts\\Database\\ModelIdentifier\":5:{s:5:\"class\";s:38:\"Filament\\Actions\\Imports\\Models\\Import\";s:2:\"id\";i:1;s:9:\"relations\";a:0:{}s:10:\"connection\";s:5:\"mysql\";s:15:\"collectionClass\";N;}s:13:\"jobConnection\";N;s:7:\"options\";a:0:{}}s:8:\"function\";s:2925:\"function () use ($columnMap, $import, $jobConnection, $options) {\n                    $import->touch(\'completed_at\');\n\n                    event(new \\Filament\\Actions\\Imports\\Events\\ImportCompleted($import, $columnMap, $options));\n\n                    if (! $import->user instanceof \\Illuminate\\Contracts\\Auth\\Authenticatable) {\n                        return;\n                    }\n\n                    $failedRowsCount = $import->getFailedRowsCount();\n\n                    \\Filament\\Notifications\\Notification::make()\n                        ->title($import->importer::getCompletedNotificationTitle($import))\n                        ->body($import->importer::getCompletedNotificationBody($import))\n                        ->when(\n                            ! $failedRowsCount,\n                            fn (\\Filament\\Notifications\\Notification $notification) => $notification->success(),\n                        )\n                        ->when(\n                            $failedRowsCount && ($failedRowsCount < $import->total_rows),\n                            fn (\\Filament\\Notifications\\Notification $notification) => $notification->warning(),\n                        )\n                        ->when(\n                            $failedRowsCount === $import->total_rows,\n                            fn (\\Filament\\Notifications\\Notification $notification) => $notification->danger(),\n                        )\n                        ->when(\n                            $failedRowsCount,\n                            fn (\\Filament\\Notifications\\Notification $notification) => $notification->actions([\n                                \\Filament\\Notifications\\Actions\\Action::make(\'downloadFailedRowsCsv\')\n                                    ->label(trans_choice(\'filament-actions::import.notifications.completed.actions.download_failed_rows_csv.label\', $failedRowsCount, [\n                                        \'count\' => \\Illuminate\\Support\\Number::format($failedRowsCount),\n                                    ]))\n                                    ->color(\'danger\')\n                                    ->url(route(\'filament.imports.failed-rows.download\', [\'import\' => $import], absolute: false), shouldOpenInNewTab: true)\n                                    ->markAsRead(),\n                            ]),\n                        )\n                        ->when(\n                            ($jobConnection === \'sync\') ||\n                                (blank($jobConnection) && (config(\'queue.default\') === \'sync\')),\n                            fn (\\Filament\\Notifications\\Notification $notification) => $notification\n                                ->persistent()\n                                ->send(),\n                            fn (\\Filament\\Notifications\\Notification $notification) => $notification->sendToDatabase($import->user, isEventDispatched: true),\n                        );\n                }\";s:5:\"scope\";s:36:\"Filament\\Tables\\Actions\\ImportAction\";s:4:\"this\";N;s:4:\"self\";s:32:\"0000000000000e0e0000000000000000\";}\";s:4:\"hash\";s:44:\"Il55kOM50sNFDkx0J1btTaCHBmZXRBPm1BERwLmAvMg=\";}}}}', NULL, 1766951427, 1766951432),
('a0b40b6c-e64b-4a5c-bed9-73d2ff339213', '', 2, 0, 0, '[]', 'a:2:{s:13:\"allowFailures\";b:1;s:7:\"finally\";a:1:{i:0;O:47:\"Laravel\\SerializableClosure\\SerializableClosure\":1:{s:12:\"serializable\";O:46:\"Laravel\\SerializableClosure\\Serializers\\Signed\":2:{s:12:\"serializable\";s:7132:\"O:46:\"Laravel\\SerializableClosure\\Serializers\\Native\":5:{s:3:\"use\";a:1:{s:4:\"next\";O:46:\"Filament\\Actions\\Exports\\Jobs\\ExportCompletion\":7:{s:11:\"\0*\0exporter\";O:35:\"App\\Filament\\Exports\\AlumniExporter\":3:{s:9:\"\0*\0export\";O:38:\"Filament\\Actions\\Exports\\Models\\Export\":33:{s:13:\"\0*\0connection\";s:5:\"mysql\";s:8:\"\0*\0table\";N;s:13:\"\0*\0primaryKey\";s:2:\"id\";s:10:\"\0*\0keyType\";s:3:\"int\";s:12:\"incrementing\";b:1;s:7:\"\0*\0with\";a:0:{}s:12:\"\0*\0withCount\";a:0:{}s:19:\"preventsLazyLoading\";b:0;s:10:\"\0*\0perPage\";i:15;s:6:\"exists\";b:1;s:18:\"wasRecentlyCreated\";b:1;s:28:\"\0*\0escapeWhenCastingToString\";b:0;s:13:\"\0*\0attributes\";a:8:{s:7:\"user_id\";i:1;s:8:\"exporter\";s:35:\"App\\Filament\\Exports\\AlumniExporter\";s:10:\"total_rows\";i:10;s:9:\"file_disk\";s:5:\"local\";s:10:\"updated_at\";s:19:\"2025-12-28 19:51:01\";s:10:\"created_at\";s:19:\"2025-12-28 19:51:01\";s:2:\"id\";i:1;s:9:\"file_name\";s:16:\"export-1-alumnis\";}s:11:\"\0*\0original\";a:8:{s:7:\"user_id\";i:1;s:8:\"exporter\";s:35:\"App\\Filament\\Exports\\AlumniExporter\";s:10:\"total_rows\";i:10;s:9:\"file_disk\";s:5:\"local\";s:10:\"updated_at\";s:19:\"2025-12-28 19:51:01\";s:10:\"created_at\";s:19:\"2025-12-28 19:51:01\";s:2:\"id\";i:1;s:9:\"file_name\";s:16:\"export-1-alumnis\";}s:10:\"\0*\0changes\";a:1:{s:9:\"file_name\";s:16:\"export-1-alumnis\";}s:11:\"\0*\0previous\";a:0:{}s:8:\"\0*\0casts\";a:4:{s:12:\"completed_at\";s:9:\"timestamp\";s:14:\"processed_rows\";s:7:\"integer\";s:10:\"total_rows\";s:7:\"integer\";s:15:\"successful_rows\";s:7:\"integer\";}s:17:\"\0*\0classCastCache\";a:0:{}s:21:\"\0*\0attributeCastCache\";a:0:{}s:13:\"\0*\0dateFormat\";N;s:10:\"\0*\0appends\";a:0:{}s:19:\"\0*\0dispatchesEvents\";a:0:{}s:14:\"\0*\0observables\";a:0:{}s:12:\"\0*\0relations\";a:0:{}s:10:\"\0*\0touches\";a:0:{}s:27:\"\0*\0relationAutoloadCallback\";N;s:26:\"\0*\0relationAutoloadContext\";N;s:10:\"timestamps\";b:1;s:13:\"usesUniqueIds\";b:0;s:9:\"\0*\0hidden\";a:0:{}s:10:\"\0*\0visible\";a:0:{}s:11:\"\0*\0fillable\";a:0:{}s:10:\"\0*\0guarded\";a:0:{}}s:12:\"\0*\0columnMap\";a:18:{s:7:\"user_id\";s:7:\"User id\";s:3:\"nim\";s:3:\"Nim\";s:8:\"username\";s:8:\"Username\";s:11:\"birth_place\";s:11:\"Birth place\";s:10:\"birth_date\";s:10:\"Birth date\";s:6:\"gender\";s:6:\"Gender\";s:7:\"address\";s:7:\"Address\";s:12:\"phone_number\";s:12:\"Phone number\";s:5:\"email\";s:5:\"Email\";s:3:\"gpa\";s:3:\"Gpa\";s:7:\"faculty\";s:7:\"Faculty\";s:5:\"major\";s:5:\"Major\";s:10:\"entry_year\";s:10:\"Entry year\";s:15:\"graduation_year\";s:15:\"Graduation year\";s:6:\"status\";s:6:\"Status\";s:12:\"job_position\";s:12:\"Job position\";s:12:\"company_name\";s:12:\"Company name\";s:4:\"foto\";s:4:\"Foto\";}s:10:\"\0*\0options\";a:0:{}}s:9:\"\0*\0export\";O:45:\"Illuminate\\Contracts\\Database\\ModelIdentifier\":5:{s:5:\"class\";s:38:\"Filament\\Actions\\Exports\\Models\\Export\";s:2:\"id\";i:1;s:9:\"relations\";a:0:{}s:10:\"connection\";s:5:\"mysql\";s:15:\"collectionClass\";N;}s:12:\"\0*\0columnMap\";a:18:{s:7:\"user_id\";s:7:\"User id\";s:3:\"nim\";s:3:\"Nim\";s:8:\"username\";s:8:\"Username\";s:11:\"birth_place\";s:11:\"Birth place\";s:10:\"birth_date\";s:10:\"Birth date\";s:6:\"gender\";s:6:\"Gender\";s:7:\"address\";s:7:\"Address\";s:12:\"phone_number\";s:12:\"Phone number\";s:5:\"email\";s:5:\"Email\";s:3:\"gpa\";s:3:\"Gpa\";s:7:\"faculty\";s:7:\"Faculty\";s:5:\"major\";s:5:\"Major\";s:10:\"entry_year\";s:10:\"Entry year\";s:15:\"graduation_year\";s:15:\"Graduation year\";s:6:\"status\";s:6:\"Status\";s:12:\"job_position\";s:12:\"Job position\";s:12:\"company_name\";s:12:\"Company name\";s:4:\"foto\";s:4:\"Foto\";}s:10:\"\0*\0formats\";a:2:{i:0;E:47:\"Filament\\Actions\\Exports\\Enums\\ExportFormat:Csv\";i:1;E:48:\"Filament\\Actions\\Exports\\Enums\\ExportFormat:Xlsx\";}s:10:\"\0*\0options\";a:0:{}s:7:\"chained\";a:1:{i:0;s:3217:\"O:44:\"Filament\\Actions\\Exports\\Jobs\\CreateXlsxFile\":4:{s:11:\"\0*\0exporter\";O:35:\"App\\Filament\\Exports\\AlumniExporter\":3:{s:9:\"\0*\0export\";O:38:\"Filament\\Actions\\Exports\\Models\\Export\":33:{s:13:\"\0*\0connection\";s:5:\"mysql\";s:8:\"\0*\0table\";N;s:13:\"\0*\0primaryKey\";s:2:\"id\";s:10:\"\0*\0keyType\";s:3:\"int\";s:12:\"incrementing\";b:1;s:7:\"\0*\0with\";a:0:{}s:12:\"\0*\0withCount\";a:0:{}s:19:\"preventsLazyLoading\";b:0;s:10:\"\0*\0perPage\";i:15;s:6:\"exists\";b:1;s:18:\"wasRecentlyCreated\";b:1;s:28:\"\0*\0escapeWhenCastingToString\";b:0;s:13:\"\0*\0attributes\";a:8:{s:7:\"user_id\";i:1;s:8:\"exporter\";s:35:\"App\\Filament\\Exports\\AlumniExporter\";s:10:\"total_rows\";i:10;s:9:\"file_disk\";s:5:\"local\";s:10:\"updated_at\";s:19:\"2025-12-28 19:51:01\";s:10:\"created_at\";s:19:\"2025-12-28 19:51:01\";s:2:\"id\";i:1;s:9:\"file_name\";s:16:\"export-1-alumnis\";}s:11:\"\0*\0original\";a:8:{s:7:\"user_id\";i:1;s:8:\"exporter\";s:35:\"App\\Filament\\Exports\\AlumniExporter\";s:10:\"total_rows\";i:10;s:9:\"file_disk\";s:5:\"local\";s:10:\"updated_at\";s:19:\"2025-12-28 19:51:01\";s:10:\"created_at\";s:19:\"2025-12-28 19:51:01\";s:2:\"id\";i:1;s:9:\"file_name\";s:16:\"export-1-alumnis\";}s:10:\"\0*\0changes\";a:1:{s:9:\"file_name\";s:16:\"export-1-alumnis\";}s:11:\"\0*\0previous\";a:0:{}s:8:\"\0*\0casts\";a:4:{s:12:\"completed_at\";s:9:\"timestamp\";s:14:\"processed_rows\";s:7:\"integer\";s:10:\"total_rows\";s:7:\"integer\";s:15:\"successful_rows\";s:7:\"integer\";}s:17:\"\0*\0classCastCache\";a:0:{}s:21:\"\0*\0attributeCastCache\";a:0:{}s:13:\"\0*\0dateFormat\";N;s:10:\"\0*\0appends\";a:0:{}s:19:\"\0*\0dispatchesEvents\";a:0:{}s:14:\"\0*\0observables\";a:0:{}s:12:\"\0*\0relations\";a:0:{}s:10:\"\0*\0touches\";a:0:{}s:27:\"\0*\0relationAutoloadCallback\";N;s:26:\"\0*\0relationAutoloadContext\";N;s:10:\"timestamps\";b:1;s:13:\"usesUniqueIds\";b:0;s:9:\"\0*\0hidden\";a:0:{}s:10:\"\0*\0visible\";a:0:{}s:11:\"\0*\0fillable\";a:0:{}s:10:\"\0*\0guarded\";a:0:{}}s:12:\"\0*\0columnMap\";a:18:{s:7:\"user_id\";s:7:\"User id\";s:3:\"nim\";s:3:\"Nim\";s:8:\"username\";s:8:\"Username\";s:11:\"birth_place\";s:11:\"Birth place\";s:10:\"birth_date\";s:10:\"Birth date\";s:6:\"gender\";s:6:\"Gender\";s:7:\"address\";s:7:\"Address\";s:12:\"phone_number\";s:12:\"Phone number\";s:5:\"email\";s:5:\"Email\";s:3:\"gpa\";s:3:\"Gpa\";s:7:\"faculty\";s:7:\"Faculty\";s:5:\"major\";s:5:\"Major\";s:10:\"entry_year\";s:10:\"Entry year\";s:15:\"graduation_year\";s:15:\"Graduation year\";s:6:\"status\";s:6:\"Status\";s:12:\"job_position\";s:12:\"Job position\";s:12:\"company_name\";s:12:\"Company name\";s:4:\"foto\";s:4:\"Foto\";}s:10:\"\0*\0options\";a:0:{}}s:9:\"\0*\0export\";O:45:\"Illuminate\\Contracts\\Database\\ModelIdentifier\":5:{s:5:\"class\";s:38:\"Filament\\Actions\\Exports\\Models\\Export\";s:2:\"id\";i:1;s:9:\"relations\";a:0:{}s:10:\"connection\";s:5:\"mysql\";s:15:\"collectionClass\";N;}s:12:\"\0*\0columnMap\";a:18:{s:7:\"user_id\";s:7:\"User id\";s:3:\"nim\";s:3:\"Nim\";s:8:\"username\";s:8:\"Username\";s:11:\"birth_place\";s:11:\"Birth place\";s:10:\"birth_date\";s:10:\"Birth date\";s:6:\"gender\";s:6:\"Gender\";s:7:\"address\";s:7:\"Address\";s:12:\"phone_number\";s:12:\"Phone number\";s:5:\"email\";s:5:\"Email\";s:3:\"gpa\";s:3:\"Gpa\";s:7:\"faculty\";s:7:\"Faculty\";s:5:\"major\";s:5:\"Major\";s:10:\"entry_year\";s:10:\"Entry year\";s:15:\"graduation_year\";s:15:\"Graduation year\";s:6:\"status\";s:6:\"Status\";s:12:\"job_position\";s:12:\"Job position\";s:12:\"company_name\";s:12:\"Company name\";s:4:\"foto\";s:4:\"Foto\";}s:10:\"\0*\0options\";a:0:{}}\";}s:19:\"chainCatchCallbacks\";a:0:{}}}s:8:\"function\";s:266:\"function (\\Illuminate\\Bus\\Batch $batch) use ($next) {\n                if (! $batch->cancelled()) {\n                    \\Illuminate\\Container\\Container::getInstance()->make(\\Illuminate\\Contracts\\Bus\\Dispatcher::class)->dispatch($next);\n                }\n            }\";s:5:\"scope\";s:27:\"Illuminate\\Bus\\ChainedBatch\";s:4:\"this\";N;s:4:\"self\";s:32:\"00000000000009740000000000000000\";}\";s:4:\"hash\";s:44:\"pqKSC78MefRaewEgLAWwiqS7IqmCa0ZSeZu8cYc6PjU=\";}}}}', NULL, 1766951462, 1766951462),
('a0b42d0a-679b-4f32-a8b2-105969130d4c', '', 1, 0, 0, '[]', 'a:2:{s:13:\"allowFailures\";b:1;s:7:\"finally\";a:1:{i:0;O:47:\"Laravel\\SerializableClosure\\SerializableClosure\":1:{s:12:\"serializable\";O:46:\"Laravel\\SerializableClosure\\Serializers\\Signed\":2:{s:12:\"serializable\";s:3952:\"O:46:\"Laravel\\SerializableClosure\\Serializers\\Native\":5:{s:3:\"use\";a:4:{s:9:\"columnMap\";a:17:{s:3:\"nim\";s:3:\"nim\";s:8:\"username\";s:8:\"username\";s:11:\"birth_place\";s:11:\"birth_place\";s:10:\"birth_date\";s:10:\"birth_date\";s:6:\"gender\";s:6:\"gender\";s:7:\"address\";s:7:\"address\";s:12:\"phone_number\";s:12:\"phone_number\";s:5:\"email\";s:5:\"email\";s:3:\"gpa\";s:3:\"gpa\";s:7:\"faculty\";s:7:\"faculty\";s:5:\"major\";s:5:\"major\";s:10:\"entry_year\";s:10:\"entry_year\";s:15:\"graduation_year\";s:15:\"graduation_year\";s:6:\"status\";s:6:\"status\";s:12:\"job_position\";s:12:\"job_position\";s:12:\"company_name\";s:12:\"company_name\";s:4:\"foto\";s:4:\"foto\";}s:6:\"import\";O:45:\"Illuminate\\Contracts\\Database\\ModelIdentifier\":5:{s:5:\"class\";s:38:\"Filament\\Actions\\Imports\\Models\\Import\";s:2:\"id\";i:1;s:9:\"relations\";a:0:{}s:10:\"connection\";s:5:\"mysql\";s:15:\"collectionClass\";N;}s:13:\"jobConnection\";N;s:7:\"options\";a:0:{}}s:8:\"function\";s:2925:\"function () use ($columnMap, $import, $jobConnection, $options) {\n                    $import->touch(\'completed_at\');\n\n                    event(new \\Filament\\Actions\\Imports\\Events\\ImportCompleted($import, $columnMap, $options));\n\n                    if (! $import->user instanceof \\Illuminate\\Contracts\\Auth\\Authenticatable) {\n                        return;\n                    }\n\n                    $failedRowsCount = $import->getFailedRowsCount();\n\n                    \\Filament\\Notifications\\Notification::make()\n                        ->title($import->importer::getCompletedNotificationTitle($import))\n                        ->body($import->importer::getCompletedNotificationBody($import))\n                        ->when(\n                            ! $failedRowsCount,\n                            fn (\\Filament\\Notifications\\Notification $notification) => $notification->success(),\n                        )\n                        ->when(\n                            $failedRowsCount && ($failedRowsCount < $import->total_rows),\n                            fn (\\Filament\\Notifications\\Notification $notification) => $notification->warning(),\n                        )\n                        ->when(\n                            $failedRowsCount === $import->total_rows,\n                            fn (\\Filament\\Notifications\\Notification $notification) => $notification->danger(),\n                        )\n                        ->when(\n                            $failedRowsCount,\n                            fn (\\Filament\\Notifications\\Notification $notification) => $notification->actions([\n                                \\Filament\\Notifications\\Actions\\Action::make(\'downloadFailedRowsCsv\')\n                                    ->label(trans_choice(\'filament-actions::import.notifications.completed.actions.download_failed_rows_csv.label\', $failedRowsCount, [\n                                        \'count\' => \\Illuminate\\Support\\Number::format($failedRowsCount),\n                                    ]))\n                                    ->color(\'danger\')\n                                    ->url(route(\'filament.imports.failed-rows.download\', [\'import\' => $import], absolute: false), shouldOpenInNewTab: true)\n                                    ->markAsRead(),\n                            ]),\n                        )\n                        ->when(\n                            ($jobConnection === \'sync\') ||\n                                (blank($jobConnection) && (config(\'queue.default\') === \'sync\')),\n                            fn (\\Filament\\Notifications\\Notification $notification) => $notification\n                                ->persistent()\n                                ->send(),\n                            fn (\\Filament\\Notifications\\Notification $notification) => $notification->sendToDatabase($import->user, isEventDispatched: true),\n                        );\n                }\";s:5:\"scope\";s:36:\"Filament\\Tables\\Actions\\ImportAction\";s:4:\"this\";N;s:4:\"self\";s:32:\"0000000000000e0e0000000000000000\";}\";s:4:\"hash\";s:44:\"Il55kOM50sNFDkx0J1btTaCHBmZXRBPm1BERwLmAvMg=\";}}}}', NULL, 1766957102, 1766957107),
('a0b6c30f-c73e-47c8-98e4-1902f8eafaa8', '', 2, 0, 0, '[]', 'a:2:{s:13:\"allowFailures\";b:1;s:7:\"finally\";a:1:{i:0;O:47:\"Laravel\\SerializableClosure\\SerializableClosure\":1:{s:12:\"serializable\";O:46:\"Laravel\\SerializableClosure\\Serializers\\Signed\":2:{s:12:\"serializable\";s:6182:\"O:46:\"Laravel\\SerializableClosure\\Serializers\\Native\":5:{s:3:\"use\";a:1:{s:4:\"next\";O:46:\"Filament\\Actions\\Exports\\Jobs\\ExportCompletion\":7:{s:11:\"\0*\0exporter\";O:50:\"App\\Filament\\Exports\\QuestionnaireResponseExporter\":3:{s:9:\"\0*\0export\";O:38:\"Filament\\Actions\\Exports\\Models\\Export\":33:{s:13:\"\0*\0connection\";s:5:\"mysql\";s:8:\"\0*\0table\";N;s:13:\"\0*\0primaryKey\";s:2:\"id\";s:10:\"\0*\0keyType\";s:3:\"int\";s:12:\"incrementing\";b:1;s:7:\"\0*\0with\";a:0:{}s:12:\"\0*\0withCount\";a:0:{}s:19:\"preventsLazyLoading\";b:0;s:10:\"\0*\0perPage\";i:15;s:6:\"exists\";b:1;s:18:\"wasRecentlyCreated\";b:1;s:28:\"\0*\0escapeWhenCastingToString\";b:0;s:13:\"\0*\0attributes\";a:8:{s:7:\"user_id\";i:1;s:8:\"exporter\";s:50:\"App\\Filament\\Exports\\QuestionnaireResponseExporter\";s:10:\"total_rows\";i:2;s:9:\"file_disk\";s:5:\"local\";s:10:\"updated_at\";s:19:\"2025-12-30 04:16:06\";s:10:\"created_at\";s:19:\"2025-12-30 04:16:05\";s:2:\"id\";i:2;s:9:\"file_name\";s:32:\"export-2-questionnaire-responses\";}s:11:\"\0*\0original\";a:8:{s:7:\"user_id\";i:1;s:8:\"exporter\";s:50:\"App\\Filament\\Exports\\QuestionnaireResponseExporter\";s:10:\"total_rows\";i:2;s:9:\"file_disk\";s:5:\"local\";s:10:\"updated_at\";s:19:\"2025-12-30 04:16:06\";s:10:\"created_at\";s:19:\"2025-12-30 04:16:05\";s:2:\"id\";i:2;s:9:\"file_name\";s:32:\"export-2-questionnaire-responses\";}s:10:\"\0*\0changes\";a:2:{s:10:\"updated_at\";s:19:\"2025-12-30 04:16:06\";s:9:\"file_name\";s:32:\"export-2-questionnaire-responses\";}s:11:\"\0*\0previous\";a:1:{s:10:\"updated_at\";s:19:\"2025-12-30 04:16:05\";}s:8:\"\0*\0casts\";a:4:{s:12:\"completed_at\";s:9:\"timestamp\";s:14:\"processed_rows\";s:7:\"integer\";s:10:\"total_rows\";s:7:\"integer\";s:15:\"successful_rows\";s:7:\"integer\";}s:17:\"\0*\0classCastCache\";a:0:{}s:21:\"\0*\0attributeCastCache\";a:0:{}s:13:\"\0*\0dateFormat\";N;s:10:\"\0*\0appends\";a:0:{}s:19:\"\0*\0dispatchesEvents\";a:0:{}s:14:\"\0*\0observables\";a:0:{}s:12:\"\0*\0relations\";a:0:{}s:10:\"\0*\0touches\";a:0:{}s:27:\"\0*\0relationAutoloadCallback\";N;s:26:\"\0*\0relationAutoloadContext\";N;s:10:\"timestamps\";b:1;s:13:\"usesUniqueIds\";b:0;s:9:\"\0*\0hidden\";a:0:{}s:10:\"\0*\0visible\";a:0:{}s:11:\"\0*\0fillable\";a:0:{}s:10:\"\0*\0guarded\";a:0:{}}s:12:\"\0*\0columnMap\";a:6:{s:19:\"questionnaire.title\";s:9:\"Kuesioner\";s:9:\"user.name\";s:6:\"Alumni\";s:15:\"user.alumni.nim\";s:3:\"NIM\";s:17:\"user.alumni.major\";s:7:\"Jurusan\";s:7:\"answers\";s:17:\"Ringkasan Jawaban\";s:12:\"submitted_at\";s:14:\"Tanggal Submit\";}s:10:\"\0*\0options\";a:0:{}}s:9:\"\0*\0export\";O:45:\"Illuminate\\Contracts\\Database\\ModelIdentifier\":5:{s:5:\"class\";s:38:\"Filament\\Actions\\Exports\\Models\\Export\";s:2:\"id\";i:2;s:9:\"relations\";a:0:{}s:10:\"connection\";s:5:\"mysql\";s:15:\"collectionClass\";N;}s:12:\"\0*\0columnMap\";a:6:{s:19:\"questionnaire.title\";s:9:\"Kuesioner\";s:9:\"user.name\";s:6:\"Alumni\";s:15:\"user.alumni.nim\";s:3:\"NIM\";s:17:\"user.alumni.major\";s:7:\"Jurusan\";s:7:\"answers\";s:17:\"Ringkasan Jawaban\";s:12:\"submitted_at\";s:14:\"Tanggal Submit\";}s:10:\"\0*\0formats\";a:2:{i:0;E:47:\"Filament\\Actions\\Exports\\Enums\\ExportFormat:Csv\";i:1;E:48:\"Filament\\Actions\\Exports\\Enums\\ExportFormat:Xlsx\";}s:10:\"\0*\0options\";a:0:{}s:7:\"chained\";a:1:{i:0;s:2742:\"O:44:\"Filament\\Actions\\Exports\\Jobs\\CreateXlsxFile\":4:{s:11:\"\0*\0exporter\";O:50:\"App\\Filament\\Exports\\QuestionnaireResponseExporter\":3:{s:9:\"\0*\0export\";O:38:\"Filament\\Actions\\Exports\\Models\\Export\":33:{s:13:\"\0*\0connection\";s:5:\"mysql\";s:8:\"\0*\0table\";N;s:13:\"\0*\0primaryKey\";s:2:\"id\";s:10:\"\0*\0keyType\";s:3:\"int\";s:12:\"incrementing\";b:1;s:7:\"\0*\0with\";a:0:{}s:12:\"\0*\0withCount\";a:0:{}s:19:\"preventsLazyLoading\";b:0;s:10:\"\0*\0perPage\";i:15;s:6:\"exists\";b:1;s:18:\"wasRecentlyCreated\";b:1;s:28:\"\0*\0escapeWhenCastingToString\";b:0;s:13:\"\0*\0attributes\";a:8:{s:7:\"user_id\";i:1;s:8:\"exporter\";s:50:\"App\\Filament\\Exports\\QuestionnaireResponseExporter\";s:10:\"total_rows\";i:2;s:9:\"file_disk\";s:5:\"local\";s:10:\"updated_at\";s:19:\"2025-12-30 04:16:06\";s:10:\"created_at\";s:19:\"2025-12-30 04:16:05\";s:2:\"id\";i:2;s:9:\"file_name\";s:32:\"export-2-questionnaire-responses\";}s:11:\"\0*\0original\";a:8:{s:7:\"user_id\";i:1;s:8:\"exporter\";s:50:\"App\\Filament\\Exports\\QuestionnaireResponseExporter\";s:10:\"total_rows\";i:2;s:9:\"file_disk\";s:5:\"local\";s:10:\"updated_at\";s:19:\"2025-12-30 04:16:06\";s:10:\"created_at\";s:19:\"2025-12-30 04:16:05\";s:2:\"id\";i:2;s:9:\"file_name\";s:32:\"export-2-questionnaire-responses\";}s:10:\"\0*\0changes\";a:2:{s:10:\"updated_at\";s:19:\"2025-12-30 04:16:06\";s:9:\"file_name\";s:32:\"export-2-questionnaire-responses\";}s:11:\"\0*\0previous\";a:1:{s:10:\"updated_at\";s:19:\"2025-12-30 04:16:05\";}s:8:\"\0*\0casts\";a:4:{s:12:\"completed_at\";s:9:\"timestamp\";s:14:\"processed_rows\";s:7:\"integer\";s:10:\"total_rows\";s:7:\"integer\";s:15:\"successful_rows\";s:7:\"integer\";}s:17:\"\0*\0classCastCache\";a:0:{}s:21:\"\0*\0attributeCastCache\";a:0:{}s:13:\"\0*\0dateFormat\";N;s:10:\"\0*\0appends\";a:0:{}s:19:\"\0*\0dispatchesEvents\";a:0:{}s:14:\"\0*\0observables\";a:0:{}s:12:\"\0*\0relations\";a:0:{}s:10:\"\0*\0touches\";a:0:{}s:27:\"\0*\0relationAutoloadCallback\";N;s:26:\"\0*\0relationAutoloadContext\";N;s:10:\"timestamps\";b:1;s:13:\"usesUniqueIds\";b:0;s:9:\"\0*\0hidden\";a:0:{}s:10:\"\0*\0visible\";a:0:{}s:11:\"\0*\0fillable\";a:0:{}s:10:\"\0*\0guarded\";a:0:{}}s:12:\"\0*\0columnMap\";a:6:{s:19:\"questionnaire.title\";s:9:\"Kuesioner\";s:9:\"user.name\";s:6:\"Alumni\";s:15:\"user.alumni.nim\";s:3:\"NIM\";s:17:\"user.alumni.major\";s:7:\"Jurusan\";s:7:\"answers\";s:17:\"Ringkasan Jawaban\";s:12:\"submitted_at\";s:14:\"Tanggal Submit\";}s:10:\"\0*\0options\";a:0:{}}s:9:\"\0*\0export\";O:45:\"Illuminate\\Contracts\\Database\\ModelIdentifier\":5:{s:5:\"class\";s:38:\"Filament\\Actions\\Exports\\Models\\Export\";s:2:\"id\";i:2;s:9:\"relations\";a:0:{}s:10:\"connection\";s:5:\"mysql\";s:15:\"collectionClass\";N;}s:12:\"\0*\0columnMap\";a:6:{s:19:\"questionnaire.title\";s:9:\"Kuesioner\";s:9:\"user.name\";s:6:\"Alumni\";s:15:\"user.alumni.nim\";s:3:\"NIM\";s:17:\"user.alumni.major\";s:7:\"Jurusan\";s:7:\"answers\";s:17:\"Ringkasan Jawaban\";s:12:\"submitted_at\";s:14:\"Tanggal Submit\";}s:10:\"\0*\0options\";a:0:{}}\";}s:19:\"chainCatchCallbacks\";a:0:{}}}s:8:\"function\";s:266:\"function (\\Illuminate\\Bus\\Batch $batch) use ($next) {\n                if (! $batch->cancelled()) {\n                    \\Illuminate\\Container\\Container::getInstance()->make(\\Illuminate\\Contracts\\Bus\\Dispatcher::class)->dispatch($next);\n                }\n            }\";s:5:\"scope\";s:27:\"Illuminate\\Bus\\ChainedBatch\";s:4:\"this\";N;s:4:\"self\";s:32:\"00000000000008fd0000000000000000\";}\";s:4:\"hash\";s:44:\"LYnbg9f8eVk3HQiI2EPE64SkSGp8Ss/aI1Ho8geoFDI=\";}}}}', NULL, 1767068170, 1767068171),
('a0bee752-6f7c-4ad8-a672-301fa493da46', '', 1, 0, 0, '[]', 'a:2:{s:13:\"allowFailures\";b:1;s:7:\"finally\";a:1:{i:0;O:47:\"Laravel\\SerializableClosure\\SerializableClosure\":1:{s:12:\"serializable\";O:46:\"Laravel\\SerializableClosure\\Serializers\\Signed\":2:{s:12:\"serializable\";s:3952:\"O:46:\"Laravel\\SerializableClosure\\Serializers\\Native\":5:{s:3:\"use\";a:4:{s:9:\"columnMap\";a:17:{s:3:\"nim\";s:3:\"nim\";s:8:\"username\";s:8:\"username\";s:11:\"birth_place\";s:11:\"birth_place\";s:10:\"birth_date\";s:10:\"birth_date\";s:6:\"gender\";s:6:\"gender\";s:7:\"address\";s:7:\"address\";s:12:\"phone_number\";s:12:\"phone_number\";s:5:\"email\";s:5:\"email\";s:3:\"gpa\";s:3:\"gpa\";s:7:\"faculty\";s:7:\"faculty\";s:5:\"major\";s:5:\"major\";s:10:\"entry_year\";s:10:\"entry_year\";s:15:\"graduation_year\";s:15:\"graduation_year\";s:6:\"status\";s:6:\"status\";s:12:\"job_position\";s:12:\"job_position\";s:12:\"company_name\";s:12:\"company_name\";s:4:\"foto\";s:4:\"foto\";}s:6:\"import\";O:45:\"Illuminate\\Contracts\\Database\\ModelIdentifier\":5:{s:5:\"class\";s:38:\"Filament\\Actions\\Imports\\Models\\Import\";s:2:\"id\";i:2;s:9:\"relations\";a:0:{}s:10:\"connection\";s:5:\"mysql\";s:15:\"collectionClass\";N;}s:13:\"jobConnection\";N;s:7:\"options\";a:0:{}}s:8:\"function\";s:2925:\"function () use ($columnMap, $import, $jobConnection, $options) {\n                    $import->touch(\'completed_at\');\n\n                    event(new \\Filament\\Actions\\Imports\\Events\\ImportCompleted($import, $columnMap, $options));\n\n                    if (! $import->user instanceof \\Illuminate\\Contracts\\Auth\\Authenticatable) {\n                        return;\n                    }\n\n                    $failedRowsCount = $import->getFailedRowsCount();\n\n                    \\Filament\\Notifications\\Notification::make()\n                        ->title($import->importer::getCompletedNotificationTitle($import))\n                        ->body($import->importer::getCompletedNotificationBody($import))\n                        ->when(\n                            ! $failedRowsCount,\n                            fn (\\Filament\\Notifications\\Notification $notification) => $notification->success(),\n                        )\n                        ->when(\n                            $failedRowsCount && ($failedRowsCount < $import->total_rows),\n                            fn (\\Filament\\Notifications\\Notification $notification) => $notification->warning(),\n                        )\n                        ->when(\n                            $failedRowsCount === $import->total_rows,\n                            fn (\\Filament\\Notifications\\Notification $notification) => $notification->danger(),\n                        )\n                        ->when(\n                            $failedRowsCount,\n                            fn (\\Filament\\Notifications\\Notification $notification) => $notification->actions([\n                                \\Filament\\Notifications\\Actions\\Action::make(\'downloadFailedRowsCsv\')\n                                    ->label(trans_choice(\'filament-actions::import.notifications.completed.actions.download_failed_rows_csv.label\', $failedRowsCount, [\n                                        \'count\' => \\Illuminate\\Support\\Number::format($failedRowsCount),\n                                    ]))\n                                    ->color(\'danger\')\n                                    ->url(route(\'filament.imports.failed-rows.download\', [\'import\' => $import], absolute: false), shouldOpenInNewTab: true)\n                                    ->markAsRead(),\n                            ]),\n                        )\n                        ->when(\n                            ($jobConnection === \'sync\') ||\n                                (blank($jobConnection) && (config(\'queue.default\') === \'sync\')),\n                            fn (\\Filament\\Notifications\\Notification $notification) => $notification\n                                ->persistent()\n                                ->send(),\n                            fn (\\Filament\\Notifications\\Notification $notification) => $notification->sendToDatabase($import->user, isEventDispatched: true),\n                        );\n                }\";s:5:\"scope\";s:36:\"Filament\\Tables\\Actions\\ImportAction\";s:4:\"this\";N;s:4:\"self\";s:32:\"0000000000000e0e0000000000000000\";}\";s:4:\"hash\";s:44:\"GCZAyQdkaHJYMqjhOd28P1Xgsw5xrUOo6jYQ3ta+7s8=\";}}}}', NULL, 1767417852, 1767417857),
('a0d35e04-f74d-41a6-aef0-e8e37eafc79e', '', 2, 0, 0, '[]', 'a:2:{s:13:\"allowFailures\";b:1;s:7:\"finally\";a:1:{i:0;O:47:\"Laravel\\SerializableClosure\\SerializableClosure\":1:{s:12:\"serializable\";O:46:\"Laravel\\SerializableClosure\\Serializers\\Signed\":2:{s:12:\"serializable\";s:7984:\"O:46:\"Laravel\\SerializableClosure\\Serializers\\Native\":5:{s:3:\"use\";a:1:{s:4:\"next\";O:46:\"Filament\\Actions\\Exports\\Jobs\\ExportCompletion\":7:{s:11:\"\0*\0exporter\";O:35:\"App\\Filament\\Exports\\AlumniExporter\":3:{s:9:\"\0*\0export\";O:38:\"Filament\\Actions\\Exports\\Models\\Export\":33:{s:13:\"\0*\0connection\";s:5:\"mysql\";s:8:\"\0*\0table\";N;s:13:\"\0*\0primaryKey\";s:2:\"id\";s:10:\"\0*\0keyType\";s:3:\"int\";s:12:\"incrementing\";b:1;s:7:\"\0*\0with\";a:0:{}s:12:\"\0*\0withCount\";a:0:{}s:19:\"preventsLazyLoading\";b:0;s:10:\"\0*\0perPage\";i:15;s:6:\"exists\";b:1;s:18:\"wasRecentlyCreated\";b:1;s:28:\"\0*\0escapeWhenCastingToString\";b:0;s:13:\"\0*\0attributes\";a:8:{s:7:\"user_id\";i:1;s:8:\"exporter\";s:35:\"App\\Filament\\Exports\\AlumniExporter\";s:10:\"total_rows\";i:13;s:9:\"file_disk\";s:5:\"local\";s:10:\"updated_at\";s:19:\"2026-01-13 09:32:35\";s:10:\"created_at\";s:19:\"2026-01-13 09:32:33\";s:2:\"id\";i:3;s:9:\"file_name\";s:16:\"export-3-alumnis\";}s:11:\"\0*\0original\";a:8:{s:7:\"user_id\";i:1;s:8:\"exporter\";s:35:\"App\\Filament\\Exports\\AlumniExporter\";s:10:\"total_rows\";i:13;s:9:\"file_disk\";s:5:\"local\";s:10:\"updated_at\";s:19:\"2026-01-13 09:32:35\";s:10:\"created_at\";s:19:\"2026-01-13 09:32:33\";s:2:\"id\";i:3;s:9:\"file_name\";s:16:\"export-3-alumnis\";}s:10:\"\0*\0changes\";a:2:{s:10:\"updated_at\";s:19:\"2026-01-13 09:32:35\";s:9:\"file_name\";s:16:\"export-3-alumnis\";}s:11:\"\0*\0previous\";a:1:{s:10:\"updated_at\";s:19:\"2026-01-13 09:32:33\";}s:8:\"\0*\0casts\";a:4:{s:12:\"completed_at\";s:9:\"timestamp\";s:14:\"processed_rows\";s:7:\"integer\";s:10:\"total_rows\";s:7:\"integer\";s:15:\"successful_rows\";s:7:\"integer\";}s:17:\"\0*\0classCastCache\";a:0:{}s:21:\"\0*\0attributeCastCache\";a:0:{}s:13:\"\0*\0dateFormat\";N;s:10:\"\0*\0appends\";a:0:{}s:19:\"\0*\0dispatchesEvents\";a:0:{}s:14:\"\0*\0observables\";a:0:{}s:12:\"\0*\0relations\";a:0:{}s:10:\"\0*\0touches\";a:0:{}s:27:\"\0*\0relationAutoloadCallback\";N;s:26:\"\0*\0relationAutoloadContext\";N;s:10:\"timestamps\";b:1;s:13:\"usesUniqueIds\";b:0;s:9:\"\0*\0hidden\";a:0:{}s:10:\"\0*\0visible\";a:0:{}s:11:\"\0*\0fillable\";a:0:{}s:10:\"\0*\0guarded\";a:0:{}}s:12:\"\0*\0columnMap\";a:21:{s:2:\"id\";s:2:\"ID\";s:3:\"nim\";s:3:\"NIM\";s:8:\"username\";s:11:\"Nama Alumni\";s:11:\"birth_place\";s:12:\"Tempat Lahir\";s:10:\"birth_date\";s:13:\"Tanggal Lahir\";s:6:\"gender\";s:13:\"Jenis Kelamin\";s:7:\"address\";s:6:\"Alamat\";s:12:\"phone_number\";s:5:\"No HP\";s:5:\"email\";s:5:\"Email\";s:3:\"gpa\";s:3:\"IPK\";s:7:\"faculty\";s:8:\"Fakultas\";s:5:\"major\";s:13:\"Program Studi\";s:10:\"entry_year\";s:11:\"Tahun Masuk\";s:15:\"graduation_year\";s:11:\"Tahun Lulus\";s:6:\"status\";s:6:\"Status\";s:14:\"category_field\";s:15:\"Kategori Bidang\";s:14:\"waiting_period\";s:11:\"Masa Tunggu\";s:12:\"job_position\";s:16:\"Posisi Pekerjaan\";s:12:\"company_name\";s:24:\"Nama Instansi/Perusahaan\";s:10:\"created_at\";s:14:\"Tanggal dibuat\";s:10:\"updated_at\";s:18:\"Tanggal diperbarui\";}s:10:\"\0*\0options\";a:0:{}}s:9:\"\0*\0export\";O:45:\"Illuminate\\Contracts\\Database\\ModelIdentifier\":5:{s:5:\"class\";s:38:\"Filament\\Actions\\Exports\\Models\\Export\";s:2:\"id\";i:3;s:9:\"relations\";a:0:{}s:10:\"connection\";s:5:\"mysql\";s:15:\"collectionClass\";N;}s:12:\"\0*\0columnMap\";a:21:{s:2:\"id\";s:2:\"ID\";s:3:\"nim\";s:3:\"NIM\";s:8:\"username\";s:11:\"Nama Alumni\";s:11:\"birth_place\";s:12:\"Tempat Lahir\";s:10:\"birth_date\";s:13:\"Tanggal Lahir\";s:6:\"gender\";s:13:\"Jenis Kelamin\";s:7:\"address\";s:6:\"Alamat\";s:12:\"phone_number\";s:5:\"No HP\";s:5:\"email\";s:5:\"Email\";s:3:\"gpa\";s:3:\"IPK\";s:7:\"faculty\";s:8:\"Fakultas\";s:5:\"major\";s:13:\"Program Studi\";s:10:\"entry_year\";s:11:\"Tahun Masuk\";s:15:\"graduation_year\";s:11:\"Tahun Lulus\";s:6:\"status\";s:6:\"Status\";s:14:\"category_field\";s:15:\"Kategori Bidang\";s:14:\"waiting_period\";s:11:\"Masa Tunggu\";s:12:\"job_position\";s:16:\"Posisi Pekerjaan\";s:12:\"company_name\";s:24:\"Nama Instansi/Perusahaan\";s:10:\"created_at\";s:14:\"Tanggal dibuat\";s:10:\"updated_at\";s:18:\"Tanggal diperbarui\";}s:10:\"\0*\0formats\";a:2:{i:0;E:47:\"Filament\\Actions\\Exports\\Enums\\ExportFormat:Csv\";i:1;E:48:\"Filament\\Actions\\Exports\\Enums\\ExportFormat:Xlsx\";}s:10:\"\0*\0options\";a:0:{}s:7:\"chained\";a:1:{i:0;s:3643:\"O:44:\"Filament\\Actions\\Exports\\Jobs\\CreateXlsxFile\":4:{s:11:\"\0*\0exporter\";O:35:\"App\\Filament\\Exports\\AlumniExporter\":3:{s:9:\"\0*\0export\";O:38:\"Filament\\Actions\\Exports\\Models\\Export\":33:{s:13:\"\0*\0connection\";s:5:\"mysql\";s:8:\"\0*\0table\";N;s:13:\"\0*\0primaryKey\";s:2:\"id\";s:10:\"\0*\0keyType\";s:3:\"int\";s:12:\"incrementing\";b:1;s:7:\"\0*\0with\";a:0:{}s:12:\"\0*\0withCount\";a:0:{}s:19:\"preventsLazyLoading\";b:0;s:10:\"\0*\0perPage\";i:15;s:6:\"exists\";b:1;s:18:\"wasRecentlyCreated\";b:1;s:28:\"\0*\0escapeWhenCastingToString\";b:0;s:13:\"\0*\0attributes\";a:8:{s:7:\"user_id\";i:1;s:8:\"exporter\";s:35:\"App\\Filament\\Exports\\AlumniExporter\";s:10:\"total_rows\";i:13;s:9:\"file_disk\";s:5:\"local\";s:10:\"updated_at\";s:19:\"2026-01-13 09:32:35\";s:10:\"created_at\";s:19:\"2026-01-13 09:32:33\";s:2:\"id\";i:3;s:9:\"file_name\";s:16:\"export-3-alumnis\";}s:11:\"\0*\0original\";a:8:{s:7:\"user_id\";i:1;s:8:\"exporter\";s:35:\"App\\Filament\\Exports\\AlumniExporter\";s:10:\"total_rows\";i:13;s:9:\"file_disk\";s:5:\"local\";s:10:\"updated_at\";s:19:\"2026-01-13 09:32:35\";s:10:\"created_at\";s:19:\"2026-01-13 09:32:33\";s:2:\"id\";i:3;s:9:\"file_name\";s:16:\"export-3-alumnis\";}s:10:\"\0*\0changes\";a:2:{s:10:\"updated_at\";s:19:\"2026-01-13 09:32:35\";s:9:\"file_name\";s:16:\"export-3-alumnis\";}s:11:\"\0*\0previous\";a:1:{s:10:\"updated_at\";s:19:\"2026-01-13 09:32:33\";}s:8:\"\0*\0casts\";a:4:{s:12:\"completed_at\";s:9:\"timestamp\";s:14:\"processed_rows\";s:7:\"integer\";s:10:\"total_rows\";s:7:\"integer\";s:15:\"successful_rows\";s:7:\"integer\";}s:17:\"\0*\0classCastCache\";a:0:{}s:21:\"\0*\0attributeCastCache\";a:0:{}s:13:\"\0*\0dateFormat\";N;s:10:\"\0*\0appends\";a:0:{}s:19:\"\0*\0dispatchesEvents\";a:0:{}s:14:\"\0*\0observables\";a:0:{}s:12:\"\0*\0relations\";a:0:{}s:10:\"\0*\0touches\";a:0:{}s:27:\"\0*\0relationAutoloadCallback\";N;s:26:\"\0*\0relationAutoloadContext\";N;s:10:\"timestamps\";b:1;s:13:\"usesUniqueIds\";b:0;s:9:\"\0*\0hidden\";a:0:{}s:10:\"\0*\0visible\";a:0:{}s:11:\"\0*\0fillable\";a:0:{}s:10:\"\0*\0guarded\";a:0:{}}s:12:\"\0*\0columnMap\";a:21:{s:2:\"id\";s:2:\"ID\";s:3:\"nim\";s:3:\"NIM\";s:8:\"username\";s:11:\"Nama Alumni\";s:11:\"birth_place\";s:12:\"Tempat Lahir\";s:10:\"birth_date\";s:13:\"Tanggal Lahir\";s:6:\"gender\";s:13:\"Jenis Kelamin\";s:7:\"address\";s:6:\"Alamat\";s:12:\"phone_number\";s:5:\"No HP\";s:5:\"email\";s:5:\"Email\";s:3:\"gpa\";s:3:\"IPK\";s:7:\"faculty\";s:8:\"Fakultas\";s:5:\"major\";s:13:\"Program Studi\";s:10:\"entry_year\";s:11:\"Tahun Masuk\";s:15:\"graduation_year\";s:11:\"Tahun Lulus\";s:6:\"status\";s:6:\"Status\";s:14:\"category_field\";s:15:\"Kategori Bidang\";s:14:\"waiting_period\";s:11:\"Masa Tunggu\";s:12:\"job_position\";s:16:\"Posisi Pekerjaan\";s:12:\"company_name\";s:24:\"Nama Instansi/Perusahaan\";s:10:\"created_at\";s:14:\"Tanggal dibuat\";s:10:\"updated_at\";s:18:\"Tanggal diperbarui\";}s:10:\"\0*\0options\";a:0:{}}s:9:\"\0*\0export\";O:45:\"Illuminate\\Contracts\\Database\\ModelIdentifier\":5:{s:5:\"class\";s:38:\"Filament\\Actions\\Exports\\Models\\Export\";s:2:\"id\";i:3;s:9:\"relations\";a:0:{}s:10:\"connection\";s:5:\"mysql\";s:15:\"collectionClass\";N;}s:12:\"\0*\0columnMap\";a:21:{s:2:\"id\";s:2:\"ID\";s:3:\"nim\";s:3:\"NIM\";s:8:\"username\";s:11:\"Nama Alumni\";s:11:\"birth_place\";s:12:\"Tempat Lahir\";s:10:\"birth_date\";s:13:\"Tanggal Lahir\";s:6:\"gender\";s:13:\"Jenis Kelamin\";s:7:\"address\";s:6:\"Alamat\";s:12:\"phone_number\";s:5:\"No HP\";s:5:\"email\";s:5:\"Email\";s:3:\"gpa\";s:3:\"IPK\";s:7:\"faculty\";s:8:\"Fakultas\";s:5:\"major\";s:13:\"Program Studi\";s:10:\"entry_year\";s:11:\"Tahun Masuk\";s:15:\"graduation_year\";s:11:\"Tahun Lulus\";s:6:\"status\";s:6:\"Status\";s:14:\"category_field\";s:15:\"Kategori Bidang\";s:14:\"waiting_period\";s:11:\"Masa Tunggu\";s:12:\"job_position\";s:16:\"Posisi Pekerjaan\";s:12:\"company_name\";s:24:\"Nama Instansi/Perusahaan\";s:10:\"created_at\";s:14:\"Tanggal dibuat\";s:10:\"updated_at\";s:18:\"Tanggal diperbarui\";}s:10:\"\0*\0options\";a:0:{}}\";}s:19:\"chainCatchCallbacks\";a:0:{}}}s:8:\"function\";s:266:\"function (\\Illuminate\\Bus\\Batch $batch) use ($next) {\n                if (! $batch->cancelled()) {\n                    \\Illuminate\\Container\\Container::getInstance()->make(\\Illuminate\\Contracts\\Bus\\Dispatcher::class)->dispatch($next);\n                }\n            }\";s:5:\"scope\";s:27:\"Illuminate\\Bus\\ChainedBatch\";s:4:\"this\";N;s:4:\"self\";s:32:\"00000000000008fd0000000000000000\";}\";s:4:\"hash\";s:44:\"eMaUhkuyXG6iehMNkJX6EIaLtEgZoE5Cwa2qF4TWSfM=\";}}}}', NULL, 1768296759, 1768296760),
('a0d36a1f-1bef-439d-b045-3647f3b37b25', '', 1, 0, 0, '[]', 'a:2:{s:13:\"allowFailures\";b:1;s:7:\"finally\";a:1:{i:0;O:47:\"Laravel\\SerializableClosure\\SerializableClosure\":1:{s:12:\"serializable\";O:46:\"Laravel\\SerializableClosure\\Serializers\\Signed\":2:{s:12:\"serializable\";s:4040:\"O:46:\"Laravel\\SerializableClosure\\Serializers\\Native\":5:{s:3:\"use\";a:4:{s:9:\"columnMap\";a:19:{s:3:\"nim\";s:3:\"nim\";s:8:\"username\";s:8:\"username\";s:11:\"birth_place\";s:11:\"birth_place\";s:10:\"birth_date\";s:10:\"birth_date\";s:6:\"gender\";s:6:\"gender\";s:7:\"address\";s:7:\"address\";s:12:\"phone_number\";s:12:\"phone_number\";s:5:\"email\";s:5:\"email\";s:3:\"gpa\";s:3:\"gpa\";s:7:\"faculty\";s:7:\"faculty\";s:5:\"major\";s:5:\"major\";s:10:\"entry_year\";s:10:\"entry_year\";s:15:\"graduation_year\";s:15:\"graduation_year\";s:6:\"status\";s:6:\"status\";s:14:\"category_field\";s:14:\"category_field\";s:14:\"waiting_period\";s:14:\"waiting_period\";s:12:\"job_position\";s:12:\"job_position\";s:12:\"company_name\";s:12:\"company_name\";s:4:\"foto\";s:4:\"foto\";}s:6:\"import\";O:45:\"Illuminate\\Contracts\\Database\\ModelIdentifier\":5:{s:5:\"class\";s:38:\"Filament\\Actions\\Imports\\Models\\Import\";s:2:\"id\";i:5;s:9:\"relations\";a:0:{}s:10:\"connection\";s:5:\"mysql\";s:15:\"collectionClass\";N;}s:13:\"jobConnection\";N;s:7:\"options\";a:0:{}}s:8:\"function\";s:2925:\"function () use ($columnMap, $import, $jobConnection, $options) {\n                    $import->touch(\'completed_at\');\n\n                    event(new \\Filament\\Actions\\Imports\\Events\\ImportCompleted($import, $columnMap, $options));\n\n                    if (! $import->user instanceof \\Illuminate\\Contracts\\Auth\\Authenticatable) {\n                        return;\n                    }\n\n                    $failedRowsCount = $import->getFailedRowsCount();\n\n                    \\Filament\\Notifications\\Notification::make()\n                        ->title($import->importer::getCompletedNotificationTitle($import))\n                        ->body($import->importer::getCompletedNotificationBody($import))\n                        ->when(\n                            ! $failedRowsCount,\n                            fn (\\Filament\\Notifications\\Notification $notification) => $notification->success(),\n                        )\n                        ->when(\n                            $failedRowsCount && ($failedRowsCount < $import->total_rows),\n                            fn (\\Filament\\Notifications\\Notification $notification) => $notification->warning(),\n                        )\n                        ->when(\n                            $failedRowsCount === $import->total_rows,\n                            fn (\\Filament\\Notifications\\Notification $notification) => $notification->danger(),\n                        )\n                        ->when(\n                            $failedRowsCount,\n                            fn (\\Filament\\Notifications\\Notification $notification) => $notification->actions([\n                                \\Filament\\Notifications\\Actions\\Action::make(\'downloadFailedRowsCsv\')\n                                    ->label(trans_choice(\'filament-actions::import.notifications.completed.actions.download_failed_rows_csv.label\', $failedRowsCount, [\n                                        \'count\' => \\Illuminate\\Support\\Number::format($failedRowsCount),\n                                    ]))\n                                    ->color(\'danger\')\n                                    ->url(route(\'filament.imports.failed-rows.download\', [\'import\' => $import], absolute: false), shouldOpenInNewTab: true)\n                                    ->markAsRead(),\n                            ]),\n                        )\n                        ->when(\n                            ($jobConnection === \'sync\') ||\n                                (blank($jobConnection) && (config(\'queue.default\') === \'sync\')),\n                            fn (\\Filament\\Notifications\\Notification $notification) => $notification\n                                ->persistent()\n                                ->send(),\n                            fn (\\Filament\\Notifications\\Notification $notification) => $notification->sendToDatabase($import->user, isEventDispatched: true),\n                        );\n                }\";s:5:\"scope\";s:36:\"Filament\\Tables\\Actions\\ImportAction\";s:4:\"this\";N;s:4:\"self\";s:32:\"0000000000000ea30000000000000000\";}\";s:4:\"hash\";s:44:\"GW5zTMvFxTdGGeVixyQBmL8UQ2Z5B8pP4RkvrmLwlwg=\";}}}}', NULL, 1768298789, 1768298794);
INSERT INTO `job_batches` (`id`, `name`, `total_jobs`, `pending_jobs`, `failed_jobs`, `failed_job_ids`, `options`, `cancelled_at`, `created_at`, `finished_at`) VALUES
('a0ef7ec8-8c61-4a5d-b019-79294f90db93', '', 2, 0, 0, '[]', 'a:2:{s:13:\"allowFailures\";b:1;s:7:\"finally\";a:1:{i:0;O:47:\"Laravel\\SerializableClosure\\SerializableClosure\":1:{s:12:\"serializable\";O:46:\"Laravel\\SerializableClosure\\Serializers\\Signed\":2:{s:12:\"serializable\";s:6006:\"O:46:\"Laravel\\SerializableClosure\\Serializers\\Native\":5:{s:3:\"use\";a:1:{s:4:\"next\";O:46:\"Filament\\Actions\\Exports\\Jobs\\ExportCompletion\":7:{s:11:\"\0*\0exporter\";O:50:\"App\\Filament\\Exports\\QuestionnaireResponseExporter\":3:{s:9:\"\0*\0export\";O:38:\"Filament\\Actions\\Exports\\Models\\Export\":33:{s:13:\"\0*\0connection\";s:5:\"mysql\";s:8:\"\0*\0table\";N;s:13:\"\0*\0primaryKey\";s:2:\"id\";s:10:\"\0*\0keyType\";s:3:\"int\";s:12:\"incrementing\";b:1;s:7:\"\0*\0with\";a:0:{}s:12:\"\0*\0withCount\";a:0:{}s:19:\"preventsLazyLoading\";b:0;s:10:\"\0*\0perPage\";i:15;s:6:\"exists\";b:1;s:18:\"wasRecentlyCreated\";b:1;s:28:\"\0*\0escapeWhenCastingToString\";b:0;s:13:\"\0*\0attributes\";a:8:{s:7:\"user_id\";i:1;s:8:\"exporter\";s:50:\"App\\Filament\\Exports\\QuestionnaireResponseExporter\";s:10:\"total_rows\";i:10;s:9:\"file_disk\";s:5:\"local\";s:10:\"updated_at\";s:19:\"2026-01-27 09:07:22\";s:10:\"created_at\";s:19:\"2026-01-27 09:07:22\";s:2:\"id\";i:4;s:9:\"file_name\";s:32:\"export-4-questionnaire-responses\";}s:11:\"\0*\0original\";a:8:{s:7:\"user_id\";i:1;s:8:\"exporter\";s:50:\"App\\Filament\\Exports\\QuestionnaireResponseExporter\";s:10:\"total_rows\";i:10;s:9:\"file_disk\";s:5:\"local\";s:10:\"updated_at\";s:19:\"2026-01-27 09:07:22\";s:10:\"created_at\";s:19:\"2026-01-27 09:07:22\";s:2:\"id\";i:4;s:9:\"file_name\";s:32:\"export-4-questionnaire-responses\";}s:10:\"\0*\0changes\";a:1:{s:9:\"file_name\";s:32:\"export-4-questionnaire-responses\";}s:11:\"\0*\0previous\";a:0:{}s:8:\"\0*\0casts\";a:4:{s:12:\"completed_at\";s:9:\"timestamp\";s:14:\"processed_rows\";s:7:\"integer\";s:10:\"total_rows\";s:7:\"integer\";s:15:\"successful_rows\";s:7:\"integer\";}s:17:\"\0*\0classCastCache\";a:0:{}s:21:\"\0*\0attributeCastCache\";a:0:{}s:13:\"\0*\0dateFormat\";N;s:10:\"\0*\0appends\";a:0:{}s:19:\"\0*\0dispatchesEvents\";a:0:{}s:14:\"\0*\0observables\";a:0:{}s:12:\"\0*\0relations\";a:0:{}s:10:\"\0*\0touches\";a:0:{}s:27:\"\0*\0relationAutoloadCallback\";N;s:26:\"\0*\0relationAutoloadContext\";N;s:10:\"timestamps\";b:1;s:13:\"usesUniqueIds\";b:0;s:9:\"\0*\0hidden\";a:0:{}s:10:\"\0*\0visible\";a:0:{}s:11:\"\0*\0fillable\";a:0:{}s:10:\"\0*\0guarded\";a:0:{}}s:12:\"\0*\0columnMap\";a:6:{s:19:\"questionnaire.title\";s:9:\"Kuesioner\";s:9:\"user.name\";s:6:\"Alumni\";s:15:\"user.alumni.nim\";s:3:\"NIM\";s:17:\"user.alumni.major\";s:7:\"Jurusan\";s:7:\"answers\";s:17:\"Ringkasan Jawaban\";s:12:\"submitted_at\";s:14:\"Tanggal Submit\";}s:10:\"\0*\0options\";a:0:{}}s:9:\"\0*\0export\";O:45:\"Illuminate\\Contracts\\Database\\ModelIdentifier\":5:{s:5:\"class\";s:38:\"Filament\\Actions\\Exports\\Models\\Export\";s:2:\"id\";i:4;s:9:\"relations\";a:0:{}s:10:\"connection\";s:5:\"mysql\";s:15:\"collectionClass\";N;}s:12:\"\0*\0columnMap\";a:6:{s:19:\"questionnaire.title\";s:9:\"Kuesioner\";s:9:\"user.name\";s:6:\"Alumni\";s:15:\"user.alumni.nim\";s:3:\"NIM\";s:17:\"user.alumni.major\";s:7:\"Jurusan\";s:7:\"answers\";s:17:\"Ringkasan Jawaban\";s:12:\"submitted_at\";s:14:\"Tanggal Submit\";}s:10:\"\0*\0formats\";a:2:{i:0;E:47:\"Filament\\Actions\\Exports\\Enums\\ExportFormat:Csv\";i:1;E:48:\"Filament\\Actions\\Exports\\Enums\\ExportFormat:Xlsx\";}s:10:\"\0*\0options\";a:0:{}s:7:\"chained\";a:1:{i:0;s:2654:\"O:44:\"Filament\\Actions\\Exports\\Jobs\\CreateXlsxFile\":4:{s:11:\"\0*\0exporter\";O:50:\"App\\Filament\\Exports\\QuestionnaireResponseExporter\":3:{s:9:\"\0*\0export\";O:38:\"Filament\\Actions\\Exports\\Models\\Export\":33:{s:13:\"\0*\0connection\";s:5:\"mysql\";s:8:\"\0*\0table\";N;s:13:\"\0*\0primaryKey\";s:2:\"id\";s:10:\"\0*\0keyType\";s:3:\"int\";s:12:\"incrementing\";b:1;s:7:\"\0*\0with\";a:0:{}s:12:\"\0*\0withCount\";a:0:{}s:19:\"preventsLazyLoading\";b:0;s:10:\"\0*\0perPage\";i:15;s:6:\"exists\";b:1;s:18:\"wasRecentlyCreated\";b:1;s:28:\"\0*\0escapeWhenCastingToString\";b:0;s:13:\"\0*\0attributes\";a:8:{s:7:\"user_id\";i:1;s:8:\"exporter\";s:50:\"App\\Filament\\Exports\\QuestionnaireResponseExporter\";s:10:\"total_rows\";i:10;s:9:\"file_disk\";s:5:\"local\";s:10:\"updated_at\";s:19:\"2026-01-27 09:07:22\";s:10:\"created_at\";s:19:\"2026-01-27 09:07:22\";s:2:\"id\";i:4;s:9:\"file_name\";s:32:\"export-4-questionnaire-responses\";}s:11:\"\0*\0original\";a:8:{s:7:\"user_id\";i:1;s:8:\"exporter\";s:50:\"App\\Filament\\Exports\\QuestionnaireResponseExporter\";s:10:\"total_rows\";i:10;s:9:\"file_disk\";s:5:\"local\";s:10:\"updated_at\";s:19:\"2026-01-27 09:07:22\";s:10:\"created_at\";s:19:\"2026-01-27 09:07:22\";s:2:\"id\";i:4;s:9:\"file_name\";s:32:\"export-4-questionnaire-responses\";}s:10:\"\0*\0changes\";a:1:{s:9:\"file_name\";s:32:\"export-4-questionnaire-responses\";}s:11:\"\0*\0previous\";a:0:{}s:8:\"\0*\0casts\";a:4:{s:12:\"completed_at\";s:9:\"timestamp\";s:14:\"processed_rows\";s:7:\"integer\";s:10:\"total_rows\";s:7:\"integer\";s:15:\"successful_rows\";s:7:\"integer\";}s:17:\"\0*\0classCastCache\";a:0:{}s:21:\"\0*\0attributeCastCache\";a:0:{}s:13:\"\0*\0dateFormat\";N;s:10:\"\0*\0appends\";a:0:{}s:19:\"\0*\0dispatchesEvents\";a:0:{}s:14:\"\0*\0observables\";a:0:{}s:12:\"\0*\0relations\";a:0:{}s:10:\"\0*\0touches\";a:0:{}s:27:\"\0*\0relationAutoloadCallback\";N;s:26:\"\0*\0relationAutoloadContext\";N;s:10:\"timestamps\";b:1;s:13:\"usesUniqueIds\";b:0;s:9:\"\0*\0hidden\";a:0:{}s:10:\"\0*\0visible\";a:0:{}s:11:\"\0*\0fillable\";a:0:{}s:10:\"\0*\0guarded\";a:0:{}}s:12:\"\0*\0columnMap\";a:6:{s:19:\"questionnaire.title\";s:9:\"Kuesioner\";s:9:\"user.name\";s:6:\"Alumni\";s:15:\"user.alumni.nim\";s:3:\"NIM\";s:17:\"user.alumni.major\";s:7:\"Jurusan\";s:7:\"answers\";s:17:\"Ringkasan Jawaban\";s:12:\"submitted_at\";s:14:\"Tanggal Submit\";}s:10:\"\0*\0options\";a:0:{}}s:9:\"\0*\0export\";O:45:\"Illuminate\\Contracts\\Database\\ModelIdentifier\":5:{s:5:\"class\";s:38:\"Filament\\Actions\\Exports\\Models\\Export\";s:2:\"id\";i:4;s:9:\"relations\";a:0:{}s:10:\"connection\";s:5:\"mysql\";s:15:\"collectionClass\";N;}s:12:\"\0*\0columnMap\";a:6:{s:19:\"questionnaire.title\";s:9:\"Kuesioner\";s:9:\"user.name\";s:6:\"Alumni\";s:15:\"user.alumni.nim\";s:3:\"NIM\";s:17:\"user.alumni.major\";s:7:\"Jurusan\";s:7:\"answers\";s:17:\"Ringkasan Jawaban\";s:12:\"submitted_at\";s:14:\"Tanggal Submit\";}s:10:\"\0*\0options\";a:0:{}}\";}s:19:\"chainCatchCallbacks\";a:0:{}}}s:8:\"function\";s:266:\"function (\\Illuminate\\Bus\\Batch $batch) use ($next) {\n                if (! $batch->cancelled()) {\n                    \\Illuminate\\Container\\Container::getInstance()->make(\\Illuminate\\Contracts\\Bus\\Dispatcher::class)->dispatch($next);\n                }\n            }\";s:5:\"scope\";s:27:\"Illuminate\\Bus\\ChainedBatch\";s:4:\"this\";N;s:4:\"self\";s:32:\"00000000000009260000000000000000\";}\";s:4:\"hash\";s:44:\"71kZRCoXQupQTIZKNNfO0c89dX8YTVq8Sxk7a46ap2A=\";}}}}', NULL, 1769504847, 1769504847),
('a0f39b8e-ec88-46f2-84f1-7fee5144d044', '', 2, 0, 0, '[]', 'a:2:{s:13:\"allowFailures\";b:1;s:7:\"finally\";a:1:{i:0;O:47:\"Laravel\\SerializableClosure\\SerializableClosure\":1:{s:12:\"serializable\";O:46:\"Laravel\\SerializableClosure\\Serializers\\Signed\":2:{s:12:\"serializable\";s:6182:\"O:46:\"Laravel\\SerializableClosure\\Serializers\\Native\":5:{s:3:\"use\";a:1:{s:4:\"next\";O:46:\"Filament\\Actions\\Exports\\Jobs\\ExportCompletion\":7:{s:11:\"\0*\0exporter\";O:50:\"App\\Filament\\Exports\\QuestionnaireResponseExporter\":3:{s:9:\"\0*\0export\";O:38:\"Filament\\Actions\\Exports\\Models\\Export\":33:{s:13:\"\0*\0connection\";s:5:\"mysql\";s:8:\"\0*\0table\";N;s:13:\"\0*\0primaryKey\";s:2:\"id\";s:10:\"\0*\0keyType\";s:3:\"int\";s:12:\"incrementing\";b:1;s:7:\"\0*\0with\";a:0:{}s:12:\"\0*\0withCount\";a:0:{}s:19:\"preventsLazyLoading\";b:0;s:10:\"\0*\0perPage\";i:15;s:6:\"exists\";b:1;s:18:\"wasRecentlyCreated\";b:1;s:28:\"\0*\0escapeWhenCastingToString\";b:0;s:13:\"\0*\0attributes\";a:8:{s:7:\"user_id\";i:1;s:8:\"exporter\";s:50:\"App\\Filament\\Exports\\QuestionnaireResponseExporter\";s:10:\"total_rows\";i:7;s:9:\"file_disk\";s:5:\"local\";s:10:\"updated_at\";s:19:\"2026-01-29 10:11:06\";s:10:\"created_at\";s:19:\"2026-01-29 10:11:05\";s:2:\"id\";i:5;s:9:\"file_name\";s:32:\"export-5-questionnaire-responses\";}s:11:\"\0*\0original\";a:8:{s:7:\"user_id\";i:1;s:8:\"exporter\";s:50:\"App\\Filament\\Exports\\QuestionnaireResponseExporter\";s:10:\"total_rows\";i:7;s:9:\"file_disk\";s:5:\"local\";s:10:\"updated_at\";s:19:\"2026-01-29 10:11:06\";s:10:\"created_at\";s:19:\"2026-01-29 10:11:05\";s:2:\"id\";i:5;s:9:\"file_name\";s:32:\"export-5-questionnaire-responses\";}s:10:\"\0*\0changes\";a:2:{s:10:\"updated_at\";s:19:\"2026-01-29 10:11:06\";s:9:\"file_name\";s:32:\"export-5-questionnaire-responses\";}s:11:\"\0*\0previous\";a:1:{s:10:\"updated_at\";s:19:\"2026-01-29 10:11:05\";}s:8:\"\0*\0casts\";a:4:{s:12:\"completed_at\";s:9:\"timestamp\";s:14:\"processed_rows\";s:7:\"integer\";s:10:\"total_rows\";s:7:\"integer\";s:15:\"successful_rows\";s:7:\"integer\";}s:17:\"\0*\0classCastCache\";a:0:{}s:21:\"\0*\0attributeCastCache\";a:0:{}s:13:\"\0*\0dateFormat\";N;s:10:\"\0*\0appends\";a:0:{}s:19:\"\0*\0dispatchesEvents\";a:0:{}s:14:\"\0*\0observables\";a:0:{}s:12:\"\0*\0relations\";a:0:{}s:10:\"\0*\0touches\";a:0:{}s:27:\"\0*\0relationAutoloadCallback\";N;s:26:\"\0*\0relationAutoloadContext\";N;s:10:\"timestamps\";b:1;s:13:\"usesUniqueIds\";b:0;s:9:\"\0*\0hidden\";a:0:{}s:10:\"\0*\0visible\";a:0:{}s:11:\"\0*\0fillable\";a:0:{}s:10:\"\0*\0guarded\";a:0:{}}s:12:\"\0*\0columnMap\";a:6:{s:19:\"questionnaire.title\";s:9:\"Kuesioner\";s:9:\"user.name\";s:6:\"Alumni\";s:15:\"user.alumni.nim\";s:3:\"NIM\";s:17:\"user.alumni.major\";s:7:\"Jurusan\";s:7:\"answers\";s:17:\"Ringkasan Jawaban\";s:12:\"submitted_at\";s:14:\"Tanggal Submit\";}s:10:\"\0*\0options\";a:0:{}}s:9:\"\0*\0export\";O:45:\"Illuminate\\Contracts\\Database\\ModelIdentifier\":5:{s:5:\"class\";s:38:\"Filament\\Actions\\Exports\\Models\\Export\";s:2:\"id\";i:5;s:9:\"relations\";a:0:{}s:10:\"connection\";s:5:\"mysql\";s:15:\"collectionClass\";N;}s:12:\"\0*\0columnMap\";a:6:{s:19:\"questionnaire.title\";s:9:\"Kuesioner\";s:9:\"user.name\";s:6:\"Alumni\";s:15:\"user.alumni.nim\";s:3:\"NIM\";s:17:\"user.alumni.major\";s:7:\"Jurusan\";s:7:\"answers\";s:17:\"Ringkasan Jawaban\";s:12:\"submitted_at\";s:14:\"Tanggal Submit\";}s:10:\"\0*\0formats\";a:2:{i:0;E:47:\"Filament\\Actions\\Exports\\Enums\\ExportFormat:Csv\";i:1;E:48:\"Filament\\Actions\\Exports\\Enums\\ExportFormat:Xlsx\";}s:10:\"\0*\0options\";a:0:{}s:7:\"chained\";a:1:{i:0;s:2742:\"O:44:\"Filament\\Actions\\Exports\\Jobs\\CreateXlsxFile\":4:{s:11:\"\0*\0exporter\";O:50:\"App\\Filament\\Exports\\QuestionnaireResponseExporter\":3:{s:9:\"\0*\0export\";O:38:\"Filament\\Actions\\Exports\\Models\\Export\":33:{s:13:\"\0*\0connection\";s:5:\"mysql\";s:8:\"\0*\0table\";N;s:13:\"\0*\0primaryKey\";s:2:\"id\";s:10:\"\0*\0keyType\";s:3:\"int\";s:12:\"incrementing\";b:1;s:7:\"\0*\0with\";a:0:{}s:12:\"\0*\0withCount\";a:0:{}s:19:\"preventsLazyLoading\";b:0;s:10:\"\0*\0perPage\";i:15;s:6:\"exists\";b:1;s:18:\"wasRecentlyCreated\";b:1;s:28:\"\0*\0escapeWhenCastingToString\";b:0;s:13:\"\0*\0attributes\";a:8:{s:7:\"user_id\";i:1;s:8:\"exporter\";s:50:\"App\\Filament\\Exports\\QuestionnaireResponseExporter\";s:10:\"total_rows\";i:7;s:9:\"file_disk\";s:5:\"local\";s:10:\"updated_at\";s:19:\"2026-01-29 10:11:06\";s:10:\"created_at\";s:19:\"2026-01-29 10:11:05\";s:2:\"id\";i:5;s:9:\"file_name\";s:32:\"export-5-questionnaire-responses\";}s:11:\"\0*\0original\";a:8:{s:7:\"user_id\";i:1;s:8:\"exporter\";s:50:\"App\\Filament\\Exports\\QuestionnaireResponseExporter\";s:10:\"total_rows\";i:7;s:9:\"file_disk\";s:5:\"local\";s:10:\"updated_at\";s:19:\"2026-01-29 10:11:06\";s:10:\"created_at\";s:19:\"2026-01-29 10:11:05\";s:2:\"id\";i:5;s:9:\"file_name\";s:32:\"export-5-questionnaire-responses\";}s:10:\"\0*\0changes\";a:2:{s:10:\"updated_at\";s:19:\"2026-01-29 10:11:06\";s:9:\"file_name\";s:32:\"export-5-questionnaire-responses\";}s:11:\"\0*\0previous\";a:1:{s:10:\"updated_at\";s:19:\"2026-01-29 10:11:05\";}s:8:\"\0*\0casts\";a:4:{s:12:\"completed_at\";s:9:\"timestamp\";s:14:\"processed_rows\";s:7:\"integer\";s:10:\"total_rows\";s:7:\"integer\";s:15:\"successful_rows\";s:7:\"integer\";}s:17:\"\0*\0classCastCache\";a:0:{}s:21:\"\0*\0attributeCastCache\";a:0:{}s:13:\"\0*\0dateFormat\";N;s:10:\"\0*\0appends\";a:0:{}s:19:\"\0*\0dispatchesEvents\";a:0:{}s:14:\"\0*\0observables\";a:0:{}s:12:\"\0*\0relations\";a:0:{}s:10:\"\0*\0touches\";a:0:{}s:27:\"\0*\0relationAutoloadCallback\";N;s:26:\"\0*\0relationAutoloadContext\";N;s:10:\"timestamps\";b:1;s:13:\"usesUniqueIds\";b:0;s:9:\"\0*\0hidden\";a:0:{}s:10:\"\0*\0visible\";a:0:{}s:11:\"\0*\0fillable\";a:0:{}s:10:\"\0*\0guarded\";a:0:{}}s:12:\"\0*\0columnMap\";a:6:{s:19:\"questionnaire.title\";s:9:\"Kuesioner\";s:9:\"user.name\";s:6:\"Alumni\";s:15:\"user.alumni.nim\";s:3:\"NIM\";s:17:\"user.alumni.major\";s:7:\"Jurusan\";s:7:\"answers\";s:17:\"Ringkasan Jawaban\";s:12:\"submitted_at\";s:14:\"Tanggal Submit\";}s:10:\"\0*\0options\";a:0:{}}s:9:\"\0*\0export\";O:45:\"Illuminate\\Contracts\\Database\\ModelIdentifier\":5:{s:5:\"class\";s:38:\"Filament\\Actions\\Exports\\Models\\Export\";s:2:\"id\";i:5;s:9:\"relations\";a:0:{}s:10:\"connection\";s:5:\"mysql\";s:15:\"collectionClass\";N;}s:12:\"\0*\0columnMap\";a:6:{s:19:\"questionnaire.title\";s:9:\"Kuesioner\";s:9:\"user.name\";s:6:\"Alumni\";s:15:\"user.alumni.nim\";s:3:\"NIM\";s:17:\"user.alumni.major\";s:7:\"Jurusan\";s:7:\"answers\";s:17:\"Ringkasan Jawaban\";s:12:\"submitted_at\";s:14:\"Tanggal Submit\";}s:10:\"\0*\0options\";a:0:{}}\";}s:19:\"chainCatchCallbacks\";a:0:{}}}s:8:\"function\";s:266:\"function (\\Illuminate\\Bus\\Batch $batch) use ($next) {\n                if (! $batch->cancelled()) {\n                    \\Illuminate\\Container\\Container::getInstance()->make(\\Illuminate\\Contracts\\Bus\\Dispatcher::class)->dispatch($next);\n                }\n            }\";s:5:\"scope\";s:27:\"Illuminate\\Bus\\ChainedBatch\";s:4:\"this\";N;s:4:\"self\";s:32:\"00000000000009260000000000000000\";}\";s:4:\"hash\";s:44:\"I6qGVrctjDxayuzSurnIBrRG8ApwB6fGyFaEQnmp9Fg=\";}}}}', NULL, 1769681473, 1769681474),
('a0f9d38d-45c0-47a3-b613-419b978c68fe', '', 2, 0, 0, '[]', 'a:2:{s:13:\"allowFailures\";b:1;s:7:\"finally\";a:1:{i:0;O:47:\"Laravel\\SerializableClosure\\SerializableClosure\":1:{s:12:\"serializable\";O:46:\"Laravel\\SerializableClosure\\Serializers\\Signed\":2:{s:12:\"serializable\";s:6646:\"O:46:\"Laravel\\SerializableClosure\\Serializers\\Native\":5:{s:3:\"use\";a:1:{s:4:\"next\";O:46:\"Filament\\Actions\\Exports\\Jobs\\ExportCompletion\":7:{s:11:\"\0*\0exporter\";O:50:\"App\\Filament\\Exports\\QuestionnaireResponseExporter\":3:{s:9:\"\0*\0export\";O:38:\"Filament\\Actions\\Exports\\Models\\Export\":33:{s:13:\"\0*\0connection\";s:5:\"mysql\";s:8:\"\0*\0table\";N;s:13:\"\0*\0primaryKey\";s:2:\"id\";s:10:\"\0*\0keyType\";s:3:\"int\";s:12:\"incrementing\";b:1;s:7:\"\0*\0with\";a:0:{}s:12:\"\0*\0withCount\";a:0:{}s:19:\"preventsLazyLoading\";b:0;s:10:\"\0*\0perPage\";i:15;s:6:\"exists\";b:1;s:18:\"wasRecentlyCreated\";b:1;s:28:\"\0*\0escapeWhenCastingToString\";b:0;s:13:\"\0*\0attributes\";a:8:{s:7:\"user_id\";i:1;s:8:\"exporter\";s:50:\"App\\Filament\\Exports\\QuestionnaireResponseExporter\";s:10:\"total_rows\";i:1;s:9:\"file_disk\";s:5:\"local\";s:10:\"updated_at\";s:19:\"2026-02-01 12:22:40\";s:10:\"created_at\";s:19:\"2026-02-01 12:22:39\";s:2:\"id\";i:6;s:9:\"file_name\";s:32:\"ekspor-6-questionnaire-responses\";}s:11:\"\0*\0original\";a:8:{s:7:\"user_id\";i:1;s:8:\"exporter\";s:50:\"App\\Filament\\Exports\\QuestionnaireResponseExporter\";s:10:\"total_rows\";i:1;s:9:\"file_disk\";s:5:\"local\";s:10:\"updated_at\";s:19:\"2026-02-01 12:22:40\";s:10:\"created_at\";s:19:\"2026-02-01 12:22:39\";s:2:\"id\";i:6;s:9:\"file_name\";s:32:\"ekspor-6-questionnaire-responses\";}s:10:\"\0*\0changes\";a:2:{s:10:\"updated_at\";s:19:\"2026-02-01 12:22:40\";s:9:\"file_name\";s:32:\"ekspor-6-questionnaire-responses\";}s:11:\"\0*\0previous\";a:1:{s:10:\"updated_at\";s:19:\"2026-02-01 12:22:39\";}s:8:\"\0*\0casts\";a:4:{s:12:\"completed_at\";s:9:\"timestamp\";s:14:\"processed_rows\";s:7:\"integer\";s:10:\"total_rows\";s:7:\"integer\";s:15:\"successful_rows\";s:7:\"integer\";}s:17:\"\0*\0classCastCache\";a:0:{}s:21:\"\0*\0attributeCastCache\";a:0:{}s:13:\"\0*\0dateFormat\";N;s:10:\"\0*\0appends\";a:0:{}s:19:\"\0*\0dispatchesEvents\";a:0:{}s:14:\"\0*\0observables\";a:0:{}s:12:\"\0*\0relations\";a:0:{}s:10:\"\0*\0touches\";a:0:{}s:27:\"\0*\0relationAutoloadCallback\";N;s:26:\"\0*\0relationAutoloadContext\";N;s:10:\"timestamps\";b:1;s:13:\"usesUniqueIds\";b:0;s:9:\"\0*\0hidden\";a:0:{}s:10:\"\0*\0visible\";a:0:{}s:11:\"\0*\0fillable\";a:0:{}s:10:\"\0*\0guarded\";a:0:{}}s:12:\"\0*\0columnMap\";a:9:{s:19:\"questionnaire.title\";s:9:\"Kuesioner\";s:9:\"user.name\";s:6:\"Alumni\";s:15:\"user.alumni.nim\";s:3:\"NIM\";s:17:\"user.alumni.major\";s:7:\"Jurusan\";s:7:\"answers\";s:17:\"Ringkasan Jawaban\";s:11:\"total_point\";s:10:\"Total Poin\";s:16:\"point_percentage\";s:10:\"Persentase\";s:14:\"point_category\";s:8:\"Kategori\";s:12:\"submitted_at\";s:14:\"Tanggal Submit\";}s:10:\"\0*\0options\";a:0:{}}s:9:\"\0*\0export\";O:45:\"Illuminate\\Contracts\\Database\\ModelIdentifier\":5:{s:5:\"class\";s:38:\"Filament\\Actions\\Exports\\Models\\Export\";s:2:\"id\";i:6;s:9:\"relations\";a:0:{}s:10:\"connection\";s:5:\"mysql\";s:15:\"collectionClass\";N;}s:12:\"\0*\0columnMap\";a:9:{s:19:\"questionnaire.title\";s:9:\"Kuesioner\";s:9:\"user.name\";s:6:\"Alumni\";s:15:\"user.alumni.nim\";s:3:\"NIM\";s:17:\"user.alumni.major\";s:7:\"Jurusan\";s:7:\"answers\";s:17:\"Ringkasan Jawaban\";s:11:\"total_point\";s:10:\"Total Poin\";s:16:\"point_percentage\";s:10:\"Persentase\";s:14:\"point_category\";s:8:\"Kategori\";s:12:\"submitted_at\";s:14:\"Tanggal Submit\";}s:10:\"\0*\0formats\";a:2:{i:0;E:47:\"Filament\\Actions\\Exports\\Enums\\ExportFormat:Csv\";i:1;E:48:\"Filament\\Actions\\Exports\\Enums\\ExportFormat:Xlsx\";}s:10:\"\0*\0options\";a:0:{}s:7:\"chained\";a:1:{i:0;s:2974:\"O:44:\"Filament\\Actions\\Exports\\Jobs\\CreateXlsxFile\":4:{s:11:\"\0*\0exporter\";O:50:\"App\\Filament\\Exports\\QuestionnaireResponseExporter\":3:{s:9:\"\0*\0export\";O:38:\"Filament\\Actions\\Exports\\Models\\Export\":33:{s:13:\"\0*\0connection\";s:5:\"mysql\";s:8:\"\0*\0table\";N;s:13:\"\0*\0primaryKey\";s:2:\"id\";s:10:\"\0*\0keyType\";s:3:\"int\";s:12:\"incrementing\";b:1;s:7:\"\0*\0with\";a:0:{}s:12:\"\0*\0withCount\";a:0:{}s:19:\"preventsLazyLoading\";b:0;s:10:\"\0*\0perPage\";i:15;s:6:\"exists\";b:1;s:18:\"wasRecentlyCreated\";b:1;s:28:\"\0*\0escapeWhenCastingToString\";b:0;s:13:\"\0*\0attributes\";a:8:{s:7:\"user_id\";i:1;s:8:\"exporter\";s:50:\"App\\Filament\\Exports\\QuestionnaireResponseExporter\";s:10:\"total_rows\";i:1;s:9:\"file_disk\";s:5:\"local\";s:10:\"updated_at\";s:19:\"2026-02-01 12:22:40\";s:10:\"created_at\";s:19:\"2026-02-01 12:22:39\";s:2:\"id\";i:6;s:9:\"file_name\";s:32:\"ekspor-6-questionnaire-responses\";}s:11:\"\0*\0original\";a:8:{s:7:\"user_id\";i:1;s:8:\"exporter\";s:50:\"App\\Filament\\Exports\\QuestionnaireResponseExporter\";s:10:\"total_rows\";i:1;s:9:\"file_disk\";s:5:\"local\";s:10:\"updated_at\";s:19:\"2026-02-01 12:22:40\";s:10:\"created_at\";s:19:\"2026-02-01 12:22:39\";s:2:\"id\";i:6;s:9:\"file_name\";s:32:\"ekspor-6-questionnaire-responses\";}s:10:\"\0*\0changes\";a:2:{s:10:\"updated_at\";s:19:\"2026-02-01 12:22:40\";s:9:\"file_name\";s:32:\"ekspor-6-questionnaire-responses\";}s:11:\"\0*\0previous\";a:1:{s:10:\"updated_at\";s:19:\"2026-02-01 12:22:39\";}s:8:\"\0*\0casts\";a:4:{s:12:\"completed_at\";s:9:\"timestamp\";s:14:\"processed_rows\";s:7:\"integer\";s:10:\"total_rows\";s:7:\"integer\";s:15:\"successful_rows\";s:7:\"integer\";}s:17:\"\0*\0classCastCache\";a:0:{}s:21:\"\0*\0attributeCastCache\";a:0:{}s:13:\"\0*\0dateFormat\";N;s:10:\"\0*\0appends\";a:0:{}s:19:\"\0*\0dispatchesEvents\";a:0:{}s:14:\"\0*\0observables\";a:0:{}s:12:\"\0*\0relations\";a:0:{}s:10:\"\0*\0touches\";a:0:{}s:27:\"\0*\0relationAutoloadCallback\";N;s:26:\"\0*\0relationAutoloadContext\";N;s:10:\"timestamps\";b:1;s:13:\"usesUniqueIds\";b:0;s:9:\"\0*\0hidden\";a:0:{}s:10:\"\0*\0visible\";a:0:{}s:11:\"\0*\0fillable\";a:0:{}s:10:\"\0*\0guarded\";a:0:{}}s:12:\"\0*\0columnMap\";a:9:{s:19:\"questionnaire.title\";s:9:\"Kuesioner\";s:9:\"user.name\";s:6:\"Alumni\";s:15:\"user.alumni.nim\";s:3:\"NIM\";s:17:\"user.alumni.major\";s:7:\"Jurusan\";s:7:\"answers\";s:17:\"Ringkasan Jawaban\";s:11:\"total_point\";s:10:\"Total Poin\";s:16:\"point_percentage\";s:10:\"Persentase\";s:14:\"point_category\";s:8:\"Kategori\";s:12:\"submitted_at\";s:14:\"Tanggal Submit\";}s:10:\"\0*\0options\";a:0:{}}s:9:\"\0*\0export\";O:45:\"Illuminate\\Contracts\\Database\\ModelIdentifier\":5:{s:5:\"class\";s:38:\"Filament\\Actions\\Exports\\Models\\Export\";s:2:\"id\";i:6;s:9:\"relations\";a:0:{}s:10:\"connection\";s:5:\"mysql\";s:15:\"collectionClass\";N;}s:12:\"\0*\0columnMap\";a:9:{s:19:\"questionnaire.title\";s:9:\"Kuesioner\";s:9:\"user.name\";s:6:\"Alumni\";s:15:\"user.alumni.nim\";s:3:\"NIM\";s:17:\"user.alumni.major\";s:7:\"Jurusan\";s:7:\"answers\";s:17:\"Ringkasan Jawaban\";s:11:\"total_point\";s:10:\"Total Poin\";s:16:\"point_percentage\";s:10:\"Persentase\";s:14:\"point_category\";s:8:\"Kategori\";s:12:\"submitted_at\";s:14:\"Tanggal Submit\";}s:10:\"\0*\0options\";a:0:{}}\";}s:19:\"chainCatchCallbacks\";a:0:{}}}s:8:\"function\";s:266:\"function (\\Illuminate\\Bus\\Batch $batch) use ($next) {\n                if (! $batch->cancelled()) {\n                    \\Illuminate\\Container\\Container::getInstance()->make(\\Illuminate\\Contracts\\Bus\\Dispatcher::class)->dispatch($next);\n                }\n            }\";s:5:\"scope\";s:27:\"Illuminate\\Bus\\ChainedBatch\";s:4:\"this\";N;s:4:\"self\";s:32:\"00000000000009360000000000000000\";}\";s:4:\"hash\";s:44:\"JkwpW1GemcJFNo8+DB1runfLgpqhc0OmIaN+AA/i1cI=\";}}}}', NULL, 1769948565, 1769948566),
('a12f7d37-1b1d-48be-afba-eeb5c33f232f', '', 1, 0, 0, '[]', 'a:2:{s:13:\"allowFailures\";b:1;s:7:\"finally\";a:1:{i:0;O:47:\"Laravel\\SerializableClosure\\SerializableClosure\":1:{s:12:\"serializable\";O:46:\"Laravel\\SerializableClosure\\Serializers\\Signed\":2:{s:12:\"serializable\";s:4040:\"O:46:\"Laravel\\SerializableClosure\\Serializers\\Native\":5:{s:3:\"use\";a:4:{s:9:\"columnMap\";a:19:{s:3:\"nim\";s:3:\"nim\";s:8:\"username\";s:8:\"username\";s:11:\"birth_place\";s:11:\"birth_place\";s:10:\"birth_date\";s:10:\"birth_date\";s:6:\"gender\";s:6:\"gender\";s:7:\"address\";s:7:\"address\";s:12:\"phone_number\";s:12:\"phone_number\";s:5:\"email\";s:5:\"email\";s:3:\"gpa\";s:3:\"gpa\";s:7:\"faculty\";s:7:\"faculty\";s:5:\"major\";s:5:\"major\";s:10:\"entry_year\";s:10:\"entry_year\";s:15:\"graduation_year\";s:15:\"graduation_year\";s:6:\"status\";s:6:\"status\";s:14:\"category_field\";s:14:\"category_field\";s:14:\"waiting_period\";s:14:\"waiting_period\";s:12:\"job_position\";s:12:\"job_position\";s:12:\"company_name\";s:12:\"company_name\";s:4:\"foto\";s:4:\"foto\";}s:6:\"import\";O:45:\"Illuminate\\Contracts\\Database\\ModelIdentifier\":5:{s:5:\"class\";s:38:\"Filament\\Actions\\Imports\\Models\\Import\";s:2:\"id\";i:7;s:9:\"relations\";a:0:{}s:10:\"connection\";s:5:\"mysql\";s:15:\"collectionClass\";N;}s:13:\"jobConnection\";N;s:7:\"options\";a:0:{}}s:8:\"function\";s:2925:\"function () use ($columnMap, $import, $jobConnection, $options) {\n                    $import->touch(\'completed_at\');\n\n                    event(new \\Filament\\Actions\\Imports\\Events\\ImportCompleted($import, $columnMap, $options));\n\n                    if (! $import->user instanceof \\Illuminate\\Contracts\\Auth\\Authenticatable) {\n                        return;\n                    }\n\n                    $failedRowsCount = $import->getFailedRowsCount();\n\n                    \\Filament\\Notifications\\Notification::make()\n                        ->title($import->importer::getCompletedNotificationTitle($import))\n                        ->body($import->importer::getCompletedNotificationBody($import))\n                        ->when(\n                            ! $failedRowsCount,\n                            fn (\\Filament\\Notifications\\Notification $notification) => $notification->success(),\n                        )\n                        ->when(\n                            $failedRowsCount && ($failedRowsCount < $import->total_rows),\n                            fn (\\Filament\\Notifications\\Notification $notification) => $notification->warning(),\n                        )\n                        ->when(\n                            $failedRowsCount === $import->total_rows,\n                            fn (\\Filament\\Notifications\\Notification $notification) => $notification->danger(),\n                        )\n                        ->when(\n                            $failedRowsCount,\n                            fn (\\Filament\\Notifications\\Notification $notification) => $notification->actions([\n                                \\Filament\\Notifications\\Actions\\Action::make(\'downloadFailedRowsCsv\')\n                                    ->label(trans_choice(\'filament-actions::import.notifications.completed.actions.download_failed_rows_csv.label\', $failedRowsCount, [\n                                        \'count\' => \\Illuminate\\Support\\Number::format($failedRowsCount),\n                                    ]))\n                                    ->color(\'danger\')\n                                    ->url(route(\'filament.imports.failed-rows.download\', [\'import\' => $import], absolute: false), shouldOpenInNewTab: true)\n                                    ->markAsRead(),\n                            ]),\n                        )\n                        ->when(\n                            ($jobConnection === \'sync\') ||\n                                (blank($jobConnection) && (config(\'queue.default\') === \'sync\')),\n                            fn (\\Filament\\Notifications\\Notification $notification) => $notification\n                                ->persistent()\n                                ->send(),\n                            fn (\\Filament\\Notifications\\Notification $notification) => $notification->sendToDatabase($import->user, isEventDispatched: true),\n                        );\n                }\";s:5:\"scope\";s:36:\"Filament\\Tables\\Actions\\ImportAction\";s:4:\"this\";N;s:4:\"self\";s:32:\"0000000000000ee00000000000000000\";}\";s:4:\"hash\";s:44:\"kimKbbWMUdhF8ZpjeuAEs3Uv23Hmwap8FBf6DnJnGtU=\";}}}}', NULL, 1772253363, 1772253369);

-- --------------------------------------------------------

--
-- Table structure for table `job_opportunities`
--

CREATE TABLE `job_opportunities` (
  `id` bigint UNSIGNED NOT NULL,
  `user_id` bigint UNSIGNED NOT NULL,
  `title` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `company` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `location` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `description` text COLLATE utf8mb4_unicode_ci NOT NULL,
  `requirements` text COLLATE utf8mb4_unicode_ci,
  `salary_range` varchar(255) COLLATE utf8mb4_unicode_ci DEFAULT NULL,
  `deadline` date DEFAULT NULL,
  `contact_info` varchar(255) COLLATE utf8mb4_unicode_ci DEFAULT NULL,
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

--
-- Dumping data for table `job_opportunities`
--

INSERT INTO `job_opportunities` (`id`, `user_id`, `title`, `company`, `location`, `description`, `requirements`, `salary_range`, `deadline`, `contact_info`, `created_at`, `updated_at`) VALUES
(1, 1, 'Junior Web Developer', 'PT Digital Solusi Nusantara', 'Bandung, Jawa Barat', '<p>&nbsp;PT Digital Solusi Nusantara membuka kesempatan bagi lulusan baru untuk bergabung sebagai Junior Web Developer. Posisi ini bertanggung jawab dalam pengembangan dan pemeliharaan aplikasi berbasis web menggunakan teknologi modern.&nbsp;</p>', '<ul><li>Lulusan D3/S1 Teknik Informatika atau Sistem Informasi</li><li>Menguasai dasar HTML, CSS, dan JavaScript</li><li>Familiar dengan framework Laravel atau React</li><li>Mampu bekerja dalam tim dan memiliki kemauan belajar tinggi</li></ul>', 'Rp 5.000.000 – Rp 7.000.000 per bulan', '2026-01-10', 'hrd@digitalsolusi.co.id', '2025-12-28 13:12:36', '2025-12-28 13:12:36'),
(3, 1, 'System Analyst Junior', 'PT Solusi Informatika Global', 'Jakarta Barat', '<blockquote>&nbsp;Perusahaan membuka posisi System Analyst Junior untuk membantu analisis kebutuhan sistem, pembuatan dokumentasi, dan koordinasi dengan tim developer dalam pengembangan aplikasi perusahaan.&nbsp;</blockquote>', '<ul><li>Lulusan S1 Sistem Informasi</li><li>Memahami SDLC dan UML</li><li>Mampu membuat dokumen kebutuhan sistem</li><li>Memiliki kemampuan komunikasi yang baik</li></ul>', 'Rp 5.000.000 – Rp 7.000.000 per bulan', '2026-03-19', 'recruitment@siginfo.co.id', '2026-01-18 03:39:09', '2026-01-18 03:39:09'),
(4, 1, 'Junior Web Developer', 'PT Digital Solusi Nusantara', 'Bandung, Jawa Barat', '<blockquote>&nbsp;PT Digital Solusi Nusantara membuka kesempatan bagi lulusan baru untuk bergabung sebagai Junior Web Developer. Posisi ini bertanggung jawab dalam pengembangan dan pemeliharaan aplikasi berbasis web menggunakan teknologi modern.&nbsp;</blockquote>', '<ul><li>Lulusan S1 Sistem Informasi</li><li>Menguasai dasar HTML, CSS, dan JavaScript</li><li>Familiar dengan framework Laravel atau React</li><li>Mampu bekerja dalam tim dan memiliki kemauan belajar tinggi</li></ul>', 'Rp 8.000.000 – Rp 10.000.000 per bulan', '2026-04-03', '0812-3456-7890', '2026-01-18 03:42:37', '2026-01-18 03:42:37');

-- --------------------------------------------------------

--
-- Table structure for table `migrations`
--

CREATE TABLE `migrations` (
  `id` int UNSIGNED NOT NULL,
  `migration` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `batch` int NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

--
-- Dumping data for table `migrations`
--

INSERT INTO `migrations` (`id`, `migration`, `batch`) VALUES
(33, '0001_01_01_000000_create_users_table', 1),
(34, '0001_01_01_000001_create_cache_table', 1),
(35, '0001_01_01_000002_create_jobs_table', 1),
(36, '2025_12_12_042503_create_alumnis_table', 1),
(37, '2025_12_23_034532_create_permission_tables', 1),
(38, '2025_12_23_051722_create_questionnaires_table', 1),
(39, '2025_12_23_074959_create_questionnaire_questions_table', 1),
(40, '2025_12_23_075229_create_questionnaire_responses_table', 1),
(41, '2025_12_23_075712_create_response_answers_table', 1),
(42, '2025_12_23_151007_create_notifications_table', 1),
(43, '2025_12_23_151222_create_imports_table', 1),
(44, '2025_12_23_151223_create_exports_table', 1),
(45, '2025_12_23_151224_create_failed_import_rows_table', 1),
(46, '2025_12_24_081956_create_job_opportunities_table', 1),
(47, '2025_12_24_082547_create_scholarships_table', 1),
(48, '2025_12_24_083203_create_internships_table', 1),
(50, '2026_01_11_084609_add_category_field_and_waiting_period_to_alumnis_table', 2),
(51, '2026_01_31_111857_create_forum_topics_table', 3),
(52, '2026_01_31_112014_create_forum_replies_table', 3),
(53, '2026_02_01_111402_add_max_point_to_questionnaire_questions_table', 4),
(54, '2026_02_01_111552_add_point_to_response_answers_table', 4);

-- --------------------------------------------------------

--
-- Table structure for table `model_has_permissions`
--

CREATE TABLE `model_has_permissions` (
  `permission_id` bigint UNSIGNED NOT NULL,
  `model_type` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `model_id` bigint UNSIGNED NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- --------------------------------------------------------

--
-- Table structure for table `model_has_roles`
--

CREATE TABLE `model_has_roles` (
  `role_id` bigint UNSIGNED NOT NULL,
  `model_type` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `model_id` bigint UNSIGNED NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

--
-- Dumping data for table `model_has_roles`
--

INSERT INTO `model_has_roles` (`role_id`, `model_type`, `model_id`) VALUES
(1, 'App\\Models\\User', 1),
(2, 'App\\Models\\User', 12),
(2, 'App\\Models\\User', 13),
(2, 'App\\Models\\User', 14),
(2, 'App\\Models\\User', 15),
(2, 'App\\Models\\User', 16),
(2, 'App\\Models\\User', 17),
(2, 'App\\Models\\User', 18),
(2, 'App\\Models\\User', 19),
(2, 'App\\Models\\User', 20),
(2, 'App\\Models\\User', 21),
(2, 'App\\Models\\User', 22),
(2, 'App\\Models\\User', 23),
(2, 'App\\Models\\User', 24),
(2, 'App\\Models\\User', 50),
(2, 'App\\Models\\User', 68),
(2, 'App\\Models\\User', 69),
(2, 'App\\Models\\User', 70),
(2, 'App\\Models\\User', 71),
(2, 'App\\Models\\User', 72),
(2, 'App\\Models\\User', 77),
(2, 'App\\Models\\User', 78),
(2, 'App\\Models\\User', 79),
(2, 'App\\Models\\User', 80),
(2, 'App\\Models\\User', 81),
(2, 'App\\Models\\User', 82),
(2, 'App\\Models\\User', 83),
(2, 'App\\Models\\User', 84),
(2, 'App\\Models\\User', 85);

-- --------------------------------------------------------

--
-- Table structure for table `notifications`
--

CREATE TABLE `notifications` (
  `id` char(36) COLLATE utf8mb4_unicode_ci NOT NULL,
  `type` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `notifiable_type` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `notifiable_id` bigint UNSIGNED NOT NULL,
  `data` text COLLATE utf8mb4_unicode_ci NOT NULL,
  `read_at` timestamp NULL DEFAULT NULL,
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

--
-- Dumping data for table `notifications`
--

INSERT INTO `notifications` (`id`, `type`, `notifiable_type`, `notifiable_id`, `data`, `read_at`, `created_at`, `updated_at`) VALUES
('00fa1033-a30d-43ab-ae0a-3e1ddac62529', 'Filament\\Notifications\\DatabaseNotification', 'App\\Models\\User', 1, '{\"actions\":[{\"name\":\"download_csv\",\"color\":null,\"event\":null,\"eventData\":[],\"dispatchDirection\":false,\"dispatchToComponent\":null,\"extraAttributes\":[],\"icon\":null,\"iconPosition\":\"before\",\"iconSize\":null,\"isOutlined\":false,\"isDisabled\":false,\"label\":\"Unduh .csv\",\"shouldClose\":false,\"shouldMarkAsRead\":true,\"shouldMarkAsUnread\":false,\"shouldOpenUrlInNewTab\":true,\"size\":\"sm\",\"tooltip\":null,\"url\":\"\\/filament\\/exports\\/6\\/download?format=csv\",\"view\":\"filament-actions::link-action\"},{\"name\":\"download_xlsx\",\"color\":null,\"event\":null,\"eventData\":[],\"dispatchDirection\":false,\"dispatchToComponent\":null,\"extraAttributes\":[],\"icon\":null,\"iconPosition\":\"before\",\"iconSize\":null,\"isOutlined\":false,\"isDisabled\":false,\"label\":\"Unduh .xlsx\",\"shouldClose\":false,\"shouldMarkAsRead\":true,\"shouldMarkAsUnread\":false,\"shouldOpenUrlInNewTab\":true,\"size\":\"sm\",\"tooltip\":null,\"url\":\"\\/filament\\/exports\\/6\\/download?format=xlsx\",\"view\":\"filament-actions::link-action\"}],\"body\":\"Your questionnaire response export has completed and 1 row exported.\",\"color\":null,\"duration\":\"persistent\",\"icon\":\"heroicon-o-check-circle\",\"iconColor\":\"success\",\"status\":\"success\",\"title\":\"Ekspor selesai\",\"view\":\"filament-notifications::notification\",\"viewData\":[],\"format\":\"filament\"}', '2026-02-01 05:23:14', '2026-02-01 05:22:47', '2026-02-01 05:23:14'),
('0b9c790f-376c-4b81-b5f7-1165999518b2', 'Filament\\Notifications\\DatabaseNotification', 'App\\Models\\User', 1, '{\"actions\":[],\"body\":\"Import alumni selesai dan 5 baris berhasil diimpor.\",\"color\":null,\"duration\":\"persistent\",\"icon\":\"heroicon-o-check-circle\",\"iconColor\":\"success\",\"status\":\"success\",\"title\":\"Import completed\",\"view\":\"filament-notifications::notification\",\"viewData\":[],\"format\":\"filament\"}', NULL, '2026-01-13 03:06:34', '2026-01-13 03:06:34'),
('15278e54-0040-48c1-aec9-0edce500b5cb', 'Filament\\Notifications\\DatabaseNotification', 'App\\Models\\User', 1, '{\"actions\":[],\"body\":\"Import alumni selesai dan 10 baris berhasil diimpor.\",\"color\":null,\"duration\":\"persistent\",\"icon\":\"heroicon-o-check-circle\",\"iconColor\":\"success\",\"status\":\"success\",\"title\":\"Import completed\",\"view\":\"filament-notifications::notification\",\"viewData\":[],\"format\":\"filament\"}', NULL, '2025-12-28 12:50:32', '2025-12-28 12:50:32'),
('16637fea-3ab9-4642-bd33-41aff0a227b6', 'Filament\\Notifications\\DatabaseNotification', 'App\\Models\\User', 1, '{\"actions\":[],\"body\":\"Import alumni selesai dan 5 baris berhasil diimpor.\",\"color\":null,\"duration\":\"persistent\",\"icon\":\"heroicon-o-check-circle\",\"iconColor\":\"success\",\"status\":\"success\",\"title\":\"Impor selesai\",\"view\":\"filament-notifications::notification\",\"viewData\":[],\"format\":\"filament\"}', NULL, '2026-02-27 21:36:10', '2026-02-27 21:36:10'),
('1cb7a1d9-e051-43e6-9090-d96f485543f3', 'Filament\\Notifications\\DatabaseNotification', 'App\\Models\\User', 1, '{\"actions\":[{\"name\":\"download_csv\",\"color\":null,\"event\":null,\"eventData\":[],\"dispatchDirection\":false,\"dispatchToComponent\":null,\"extraAttributes\":[],\"icon\":null,\"iconPosition\":\"before\",\"iconSize\":null,\"isOutlined\":false,\"isDisabled\":false,\"label\":\"Download .csv\",\"shouldClose\":false,\"shouldMarkAsRead\":true,\"shouldMarkAsUnread\":false,\"shouldOpenUrlInNewTab\":true,\"size\":\"sm\",\"tooltip\":null,\"url\":\"\\/filament\\/exports\\/1\\/download?format=csv\",\"view\":\"filament-actions::link-action\"},{\"name\":\"download_xlsx\",\"color\":null,\"event\":null,\"eventData\":[],\"dispatchDirection\":false,\"dispatchToComponent\":null,\"extraAttributes\":[],\"icon\":null,\"iconPosition\":\"before\",\"iconSize\":null,\"isOutlined\":false,\"isDisabled\":false,\"label\":\"Download .xlsx\",\"shouldClose\":false,\"shouldMarkAsRead\":true,\"shouldMarkAsUnread\":false,\"shouldOpenUrlInNewTab\":true,\"size\":\"sm\",\"tooltip\":null,\"url\":\"\\/filament\\/exports\\/1\\/download?format=xlsx\",\"view\":\"filament-actions::link-action\"}],\"body\":\"Your alumni export has completed and 10 rows exported.\",\"color\":null,\"duration\":\"persistent\",\"icon\":\"heroicon-o-check-circle\",\"iconColor\":\"success\",\"status\":\"success\",\"title\":\"Export completed\",\"view\":\"filament-notifications::notification\",\"viewData\":[],\"format\":\"filament\"}', '2025-12-28 12:51:13', '2025-12-28 12:51:02', '2025-12-28 12:51:13'),
('2dd902d2-9c84-48a6-88d7-c02f43655e38', 'Filament\\Notifications\\DatabaseNotification', 'App\\Models\\User', 1, '{\"actions\":[{\"name\":\"download_csv\",\"color\":null,\"event\":null,\"eventData\":[],\"dispatchDirection\":false,\"dispatchToComponent\":null,\"extraAttributes\":[],\"icon\":null,\"iconPosition\":\"before\",\"iconSize\":null,\"isOutlined\":false,\"isDisabled\":false,\"label\":\"Download .csv\",\"shouldClose\":false,\"shouldMarkAsRead\":true,\"shouldMarkAsUnread\":false,\"shouldOpenUrlInNewTab\":true,\"size\":\"sm\",\"tooltip\":null,\"url\":\"\\/filament\\/exports\\/5\\/download?format=csv\",\"view\":\"filament-actions::link-action\"},{\"name\":\"download_xlsx\",\"color\":null,\"event\":null,\"eventData\":[],\"dispatchDirection\":false,\"dispatchToComponent\":null,\"extraAttributes\":[],\"icon\":null,\"iconPosition\":\"before\",\"iconSize\":null,\"isOutlined\":false,\"isDisabled\":false,\"label\":\"Download .xlsx\",\"shouldClose\":false,\"shouldMarkAsRead\":true,\"shouldMarkAsUnread\":false,\"shouldOpenUrlInNewTab\":true,\"size\":\"sm\",\"tooltip\":null,\"url\":\"\\/filament\\/exports\\/5\\/download?format=xlsx\",\"view\":\"filament-actions::link-action\"}],\"body\":\"Your questionnaire response export has completed and 7 rows exported.\",\"color\":null,\"duration\":\"persistent\",\"icon\":\"heroicon-o-check-circle\",\"iconColor\":\"success\",\"status\":\"success\",\"title\":\"Export completed\",\"view\":\"filament-notifications::notification\",\"viewData\":[],\"format\":\"filament\"}', '2026-01-29 03:11:32', '2026-01-29 03:11:15', '2026-01-29 03:11:32'),
('4d6bef7e-bfef-445f-b0b6-b6977daf00bd', 'Filament\\Notifications\\DatabaseNotification', 'App\\Models\\User', 1, '{\"actions\":[{\"name\":\"download_csv\",\"color\":null,\"event\":null,\"eventData\":[],\"dispatchDirection\":false,\"dispatchToComponent\":null,\"extraAttributes\":[],\"icon\":null,\"iconPosition\":\"before\",\"iconSize\":null,\"isOutlined\":false,\"isDisabled\":false,\"label\":\"Download .csv\",\"shouldClose\":false,\"shouldMarkAsRead\":true,\"shouldMarkAsUnread\":false,\"shouldOpenUrlInNewTab\":true,\"size\":\"sm\",\"tooltip\":null,\"url\":\"\\/filament\\/exports\\/2\\/download?format=csv\",\"view\":\"filament-actions::link-action\"},{\"name\":\"download_xlsx\",\"color\":null,\"event\":null,\"eventData\":[],\"dispatchDirection\":false,\"dispatchToComponent\":null,\"extraAttributes\":[],\"icon\":null,\"iconPosition\":\"before\",\"iconSize\":null,\"isOutlined\":false,\"isDisabled\":false,\"label\":\"Download .xlsx\",\"shouldClose\":false,\"shouldMarkAsRead\":true,\"shouldMarkAsUnread\":false,\"shouldOpenUrlInNewTab\":true,\"size\":\"sm\",\"tooltip\":null,\"url\":\"\\/filament\\/exports\\/2\\/download?format=xlsx\",\"view\":\"filament-actions::link-action\"}],\"body\":\"Your questionnaire response export has completed and 2 rows exported.\",\"color\":null,\"duration\":\"persistent\",\"icon\":\"heroicon-o-check-circle\",\"iconColor\":\"success\",\"status\":\"success\",\"title\":\"Export completed\",\"view\":\"filament-notifications::notification\",\"viewData\":[],\"format\":\"filament\"}', '2025-12-29 21:16:42', '2025-12-29 21:16:12', '2025-12-29 21:16:42'),
('a5fe8313-831f-4962-be14-3104c1bb7a1e', 'Filament\\Notifications\\DatabaseNotification', 'App\\Models\\User', 1, '{\"actions\":[{\"name\":\"download_csv\",\"color\":null,\"event\":null,\"eventData\":[],\"dispatchDirection\":false,\"dispatchToComponent\":null,\"extraAttributes\":[],\"icon\":null,\"iconPosition\":\"before\",\"iconSize\":null,\"isOutlined\":false,\"isDisabled\":false,\"label\":\"Download .csv\",\"shouldClose\":false,\"shouldMarkAsRead\":true,\"shouldMarkAsUnread\":false,\"shouldOpenUrlInNewTab\":true,\"size\":\"sm\",\"tooltip\":null,\"url\":\"\\/filament\\/exports\\/4\\/download?format=csv\",\"view\":\"filament-actions::link-action\"},{\"name\":\"download_xlsx\",\"color\":null,\"event\":null,\"eventData\":[],\"dispatchDirection\":false,\"dispatchToComponent\":null,\"extraAttributes\":[],\"icon\":null,\"iconPosition\":\"before\",\"iconSize\":null,\"isOutlined\":false,\"isDisabled\":false,\"label\":\"Download .xlsx\",\"shouldClose\":false,\"shouldMarkAsRead\":true,\"shouldMarkAsUnread\":false,\"shouldOpenUrlInNewTab\":true,\"size\":\"sm\",\"tooltip\":null,\"url\":\"\\/filament\\/exports\\/4\\/download?format=xlsx\",\"view\":\"filament-actions::link-action\"}],\"body\":\"Your questionnaire response export has completed and 10 rows exported.\",\"color\":null,\"duration\":\"persistent\",\"icon\":\"heroicon-o-check-circle\",\"iconColor\":\"success\",\"status\":\"success\",\"title\":\"Export completed\",\"view\":\"filament-notifications::notification\",\"viewData\":[],\"format\":\"filament\"}', NULL, '2026-01-27 02:07:28', '2026-01-27 02:07:28'),
('a9827db2-a578-4d6f-98d9-59d12da1eb41', 'Filament\\Notifications\\DatabaseNotification', 'App\\Models\\User', 1, '{\"actions\":[],\"body\":\"Import alumni selesai dan 10 baris berhasil diimpor.\",\"color\":null,\"duration\":\"persistent\",\"icon\":\"heroicon-o-check-circle\",\"iconColor\":\"success\",\"status\":\"success\",\"title\":\"Import completed\",\"view\":\"filament-notifications::notification\",\"viewData\":[],\"format\":\"filament\"}', NULL, '2026-01-02 22:24:18', '2026-01-02 22:24:18'),
('d212c38d-6d78-4336-aa89-b374455a302e', 'Filament\\Notifications\\DatabaseNotification', 'App\\Models\\User', 1, '{\"actions\":[{\"name\":\"download_csv\",\"color\":null,\"event\":null,\"eventData\":[],\"dispatchDirection\":false,\"dispatchToComponent\":null,\"extraAttributes\":[],\"icon\":null,\"iconPosition\":\"before\",\"iconSize\":null,\"isOutlined\":false,\"isDisabled\":false,\"label\":\"Download .csv\",\"shouldClose\":false,\"shouldMarkAsRead\":true,\"shouldMarkAsUnread\":false,\"shouldOpenUrlInNewTab\":true,\"size\":\"sm\",\"tooltip\":null,\"url\":\"\\/filament\\/exports\\/3\\/download?format=csv\",\"view\":\"filament-actions::link-action\"},{\"name\":\"download_xlsx\",\"color\":null,\"event\":null,\"eventData\":[],\"dispatchDirection\":false,\"dispatchToComponent\":null,\"extraAttributes\":[],\"icon\":null,\"iconPosition\":\"before\",\"iconSize\":null,\"isOutlined\":false,\"isDisabled\":false,\"label\":\"Download .xlsx\",\"shouldClose\":false,\"shouldMarkAsRead\":true,\"shouldMarkAsUnread\":false,\"shouldOpenUrlInNewTab\":true,\"size\":\"sm\",\"tooltip\":null,\"url\":\"\\/filament\\/exports\\/3\\/download?format=xlsx\",\"view\":\"filament-actions::link-action\"}],\"body\":\"Your alumni export has completed and 13 rows exported.\",\"color\":null,\"duration\":\"persistent\",\"icon\":\"heroicon-o-check-circle\",\"iconColor\":\"success\",\"status\":\"success\",\"title\":\"Export completed\",\"view\":\"filament-notifications::notification\",\"viewData\":[],\"format\":\"filament\"}', '2026-01-13 02:33:00', '2026-01-13 02:32:41', '2026-01-13 02:33:00'),
('f8296e68-1d49-4391-b406-bf0a840b2eb6', 'Filament\\Notifications\\DatabaseNotification', 'App\\Models\\User', 1, '{\"actions\":[],\"body\":\"Import alumni selesai dan 10 baris berhasil diimpor.\",\"color\":null,\"duration\":\"persistent\",\"icon\":\"heroicon-o-check-circle\",\"iconColor\":\"success\",\"status\":\"success\",\"title\":\"Import completed\",\"view\":\"filament-notifications::notification\",\"viewData\":[],\"format\":\"filament\"}', NULL, '2025-12-28 14:25:08', '2025-12-28 14:25:08');

-- --------------------------------------------------------

--
-- Table structure for table `password_reset_tokens`
--

CREATE TABLE `password_reset_tokens` (
  `email` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `token` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `created_at` timestamp NULL DEFAULT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- --------------------------------------------------------

--
-- Table structure for table `permissions`
--

CREATE TABLE `permissions` (
  `id` bigint UNSIGNED NOT NULL,
  `name` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `guard_name` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

--
-- Dumping data for table `permissions`
--

INSERT INTO `permissions` (`id`, `name`, `guard_name`, `created_at`, `updated_at`) VALUES
(1, 'view_alumni', 'web', '2025-12-28 12:49:06', '2025-12-28 12:49:06'),
(2, 'create_alumni', 'web', '2025-12-28 12:49:06', '2025-12-28 12:49:06'),
(3, 'edit_alumni', 'web', '2025-12-28 12:49:06', '2025-12-28 12:49:06'),
(4, 'delete_alumni', 'web', '2025-12-28 12:49:06', '2025-12-28 12:49:06'),
(5, 'import_alumni', 'web', '2025-12-28 12:49:06', '2025-12-28 12:49:06'),
(6, 'export_alumni', 'web', '2025-12-28 12:49:06', '2025-12-28 12:49:06'),
(7, 'view_questionnaire', 'web', '2025-12-28 12:49:06', '2025-12-28 12:49:06'),
(8, 'create_questionnaire', 'web', '2025-12-28 12:49:06', '2025-12-28 12:49:06'),
(9, 'edit_questionnaire', 'web', '2025-12-28 12:49:06', '2025-12-28 12:49:06'),
(10, 'delete_questionnaire', 'web', '2025-12-28 12:49:06', '2025-12-28 12:49:06'),
(11, 'view_responses', 'web', '2025-12-28 12:49:06', '2025-12-28 12:49:06'),
(12, 'export_responses', 'web', '2025-12-28 12:49:06', '2025-12-28 12:49:06'),
(13, 'fill_questionnaire', 'web', '2025-12-28 12:49:06', '2025-12-28 12:49:06'),
(14, 'view_job', 'web', '2025-12-28 12:49:06', '2025-12-28 12:49:06'),
(15, 'create_job', 'web', '2025-12-28 12:49:06', '2025-12-28 12:49:06'),
(16, 'edit_own_job', 'web', '2025-12-28 12:49:06', '2025-12-28 12:49:06'),
(17, 'delete_own_job', 'web', '2025-12-28 12:49:06', '2025-12-28 12:49:06'),
(18, 'view_scholarship', 'web', '2025-12-28 12:49:06', '2025-12-28 12:49:06'),
(19, 'create_scholarship', 'web', '2025-12-28 12:49:06', '2025-12-28 12:49:06'),
(20, 'edit_own_scholarship', 'web', '2025-12-28 12:49:06', '2025-12-28 12:49:06'),
(21, 'delete_own_scholarship', 'web', '2025-12-28 12:49:06', '2025-12-28 12:49:06'),
(22, 'view_internship', 'web', '2025-12-28 12:49:06', '2025-12-28 12:49:06'),
(23, 'create_internship', 'web', '2025-12-28 12:49:06', '2025-12-28 12:49:06'),
(24, 'edit_own_internship', 'web', '2025-12-28 12:49:06', '2025-12-28 12:49:06'),
(25, 'delete_own_internship', 'web', '2025-12-28 12:49:06', '2025-12-28 12:49:06'),
(26, 'view_admin_dashboard', 'web', '2025-12-28 12:49:06', '2025-12-28 12:49:06'),
(27, 'view_alumni_dashboard', 'web', '2025-12-28 12:49:06', '2025-12-28 12:49:06'),
(28, 'view_forum', 'web', '2026-01-31 05:39:48', '2026-01-31 05:39:48'),
(29, 'create_forum_topic', 'web', '2026-01-31 05:39:49', '2026-01-31 05:39:49'),
(30, 'edit_own_forum_topic', 'web', '2026-01-31 05:39:49', '2026-01-31 05:39:49'),
(31, 'delete_own_forum_topic', 'web', '2026-01-31 05:39:49', '2026-01-31 05:39:49'),
(32, 'reply_forum_topic', 'web', '2026-01-31 05:39:49', '2026-01-31 05:39:49'),
(33, 'moderate_forum', 'web', '2026-01-31 05:39:49', '2026-01-31 05:39:49');

-- --------------------------------------------------------

--
-- Table structure for table `questionnaires`
--

CREATE TABLE `questionnaires` (
  `id` bigint UNSIGNED NOT NULL,
  `title` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `description` text COLLATE utf8mb4_unicode_ci,
  `tahun_periode` year NOT NULL,
  `is_active` tinyint(1) NOT NULL DEFAULT '1',
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

--
-- Dumping data for table `questionnaires`
--

INSERT INTO `questionnaires` (`id`, `title`, `description`, `tahun_periode`, `is_active`, `created_at`, `updated_at`) VALUES
(8, 'KUESIONER TRACER STUDY ALUMNI', 'Kuesioner ini digunakan untuk memperoleh umpan balik alumni sebagai bahan evaluasi dan pengembangan kurikulum serta peningkatan mutu program studi.\nSetiap pernyataan dijawab dengan satu pilihan berikut:\n1 = Sangat Tidak Setuju\n2 = Tidak Setuju\n3 = Netral\n4 = Setuju\n5 = Sangat Setuju', 2026, 1, '2026-02-01 04:46:19', '2026-02-01 04:46:19'),
(9, 'Pendidikan dan Karir', 'Bagian ini bertujuan untuk mengetahui perkembangan pendidikan dan karier alumni setelah lulus.', 2025, 1, '2026-02-27 20:25:23', '2026-02-27 21:59:00'),
(10, 'Penilaian Program Studi', 'Bagian ini bertujuan untuk mengevaluasi kualitas pendidikan dan fasilitas program studi.', 2026, 1, '2026-02-27 20:47:39', '2026-02-27 20:57:47');

-- --------------------------------------------------------

--
-- Table structure for table `questionnaire_questions`
--

CREATE TABLE `questionnaire_questions` (
  `id` bigint UNSIGNED NOT NULL,
  `questionnaire_id` bigint UNSIGNED NOT NULL,
  `question_text` text COLLATE utf8mb4_unicode_ci NOT NULL,
  `question_type` enum('text','textarea','radio','checkbox','select') COLLATE utf8mb4_unicode_ci NOT NULL,
  `options` json DEFAULT NULL,
  `is_required` tinyint(1) NOT NULL DEFAULT '0',
  `max_point` int NOT NULL DEFAULT '5' COMMENT 'Maksimal poin yang bisa didapat dari pertanyaan ini',
  `order` int NOT NULL DEFAULT '0',
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

--
-- Dumping data for table `questionnaire_questions`
--

INSERT INTO `questionnaire_questions` (`id`, `questionnaire_id`, `question_text`, `question_type`, `options`, `is_required`, `max_point`, `order`, `created_at`, `updated_at`) VALUES
(20, 8, 'Kompetensi yang saya peroleh selama kuliah sesuai dengan kebutuhan pekerjaan saya saat ini.', 'radio', '[\"Sangat Tidak Setuju\", \"Tidak Setuju\", \"Netral\", \"Setuju\", \"Sangat Setuju\"]', 1, 5, 1, '2026-02-01 04:50:26', '2026-02-01 04:50:26'),
(21, 8, 'Kurikulum program studi relevan dengan perkembangan dunia kerja.', 'radio', '[\"Sangat Tidak Setuju\", \"Tidak Setuju\", \"Netral\", \"Setuju\", \"Sangat Setuju\"]', 1, 5, 2, '2026-02-01 04:52:18', '2026-02-01 04:52:18'),
(22, 8, 'Mata kuliah yang saya peroleh mendukung pelaksanaan tugas di tempat kerja.', 'radio', '[\"Sangat Tidak Setuju\", \"Tidak Setuju\", \"Netral\", \"Setuju\", \"Sangat Setuju\"]', 1, 5, 3, '2026-02-01 04:54:57', '2026-02-01 04:54:57'),
(23, 8, 'Program studi membekali saya dengan kemampuan berpikir kritis.', 'radio', '[\"Sangat Tidak Setuju\", \"Tidak Setuju\", \"Netral\", \"Setuju\", \"Sangat Setuju\"]', 1, 5, 4, '2026-02-01 04:56:52', '2026-02-01 04:56:52'),
(24, 8, 'Program studi membekali saya dengan kemampuan pemecahan masalah.', 'radio', '[\"Sangat Tidak Setuju\", \"Tidak Setuju\", \"Netral\", \"Setuju\", \"Sangat Setuju\"]', 1, 5, 5, '2026-02-01 05:00:25', '2026-02-01 05:00:25'),
(25, 9, 'Apakah Anda melanjutkan studi setelah lulus?', 'radio', '[\"Ya\", \"Tidak\"]', 1, 5, 1, '2026-02-27 20:27:19', '2026-02-27 20:27:19'),
(26, 9, 'Jika ya, program studi yang diambil', 'text', NULL, 1, 5, 2, '2026-02-27 20:27:49', '2026-02-27 20:27:49'),
(27, 9, 'Apakah Anda saat ini bekerja?', 'radio', '[\"Ya, sesuai bidang studi\", \"Ya, tidak sesuai bidang studi\", \"Tidak bekerja\"]', 1, 5, 3, '2026-02-27 20:29:14', '2026-02-27 20:29:14'),
(28, 9, 'Posisi/jabatan terakhir Anda', 'text', NULL, 1, 5, 4, '2026-02-27 20:29:34', '2026-02-27 20:29:34'),
(29, 9, 'Nama perusahaan/instansi tempat bekerja', 'text', NULL, 1, 5, 5, '2026-02-27 20:29:55', '2026-02-27 20:31:23'),
(30, 9, 'Seberapa sesuai pekerjaan Anda dengan bidang studi?', 'radio', '[\"Sangat sesuai\", \"Sesuai\", \"Kurang sesuai\", \"Tidak sesuai\"]', 1, 5, 6, '2026-02-27 20:31:06', '2026-02-27 20:31:06'),
(31, 10, 'Seberapa puas Anda terhadap kualitas pendidikan di program studi?', 'radio', '[\"Sangat puas\", \"Puas\", \"Cukup\", \"Kurang\"]', 1, 5, 1, '2026-02-27 20:51:42', '2026-02-27 20:51:42'),
(32, 10, 'Bagaimana kualitas fasilitas pendukung pembelajaran (laboratorium, perpustakaan, dll)?', 'radio', '[\"Sangat baik\", \"Baik\", \"Cukup\", \"Kurang\"]', 1, 5, 2, '2026-02-27 20:52:38', '2026-02-27 20:52:38'),
(33, 10, 'Apakah kurikulum program studi relevan dengan kebutuhan industri saat ini?', 'radio', '[\"Sangat relevan\", \"Relevan\", \"Kurang Relevan\", \"Tidak Relevan\"]', 1, 5, 3, '2026-02-27 20:53:58', '2026-02-27 20:53:58'),
(34, 10, 'Seberapa besar program studi membantu Anda dalam mengembangkan soft skills (komunikasi, kepemimpinan, kerja tim, dll)?', 'radio', '[\"Sangat Besar\", \"Besar\", \"Cukup\", \"Kurang\"]', 1, 5, 4, '2026-02-27 20:54:50', '2026-02-27 20:54:50');

-- --------------------------------------------------------

--
-- Table structure for table `questionnaire_responses`
--

CREATE TABLE `questionnaire_responses` (
  `id` bigint UNSIGNED NOT NULL,
  `questionnaire_id` bigint UNSIGNED NOT NULL,
  `user_id` bigint UNSIGNED NOT NULL,
  `submitted_at` timestamp NOT NULL,
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

--
-- Dumping data for table `questionnaire_responses`
--

INSERT INTO `questionnaire_responses` (`id`, `questionnaire_id`, `user_id`, `submitted_at`, `created_at`, `updated_at`) VALUES
(12, 8, 78, '2026-02-01 05:15:38', '2026-02-01 05:15:38', '2026-02-01 05:15:38'),
(14, 8, 12, '2026-02-27 20:39:42', '2026-02-27 20:39:42', '2026-02-27 20:39:42'),
(16, 10, 12, '2026-02-27 20:58:54', '2026-02-27 20:58:54', '2026-02-27 20:58:54'),
(17, 8, 14, '2026-02-27 21:00:14', '2026-02-27 21:00:14', '2026-02-27 21:00:14'),
(19, 10, 14, '2026-02-27 21:01:53', '2026-02-27 21:01:53', '2026-02-27 21:01:53');

-- --------------------------------------------------------

--
-- Table structure for table `response_answers`
--

CREATE TABLE `response_answers` (
  `id` bigint UNSIGNED NOT NULL,
  `response_id` bigint UNSIGNED NOT NULL,
  `question_id` bigint UNSIGNED NOT NULL,
  `answer_text` text COLLATE utf8mb4_unicode_ci,
  `point` int NOT NULL DEFAULT '0' COMMENT 'Poin yang didapat dari jawaban ini',
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

--
-- Dumping data for table `response_answers`
--

INSERT INTO `response_answers` (`id`, `response_id`, `question_id`, `answer_text`, `point`, `created_at`, `updated_at`) VALUES
(44, 12, 20, 'Setuju', 4, '2026-02-01 05:15:38', '2026-02-01 05:15:38'),
(45, 12, 21, 'Netral', 3, '2026-02-01 05:15:38', '2026-02-01 05:15:38'),
(46, 12, 22, 'Netral', 3, '2026-02-01 05:15:38', '2026-02-01 05:15:38'),
(47, 12, 23, 'Setuju', 4, '2026-02-01 05:15:38', '2026-02-01 05:15:38'),
(48, 12, 24, 'Setuju', 4, '2026-02-01 05:15:38', '2026-02-01 05:15:38'),
(54, 14, 20, 'Setuju', 4, '2026-02-27 20:39:42', '2026-02-27 20:39:42'),
(55, 14, 21, 'Setuju', 4, '2026-02-27 20:39:42', '2026-02-27 20:39:42'),
(56, 14, 22, 'Setuju', 4, '2026-02-27 20:39:42', '2026-02-27 20:39:42'),
(57, 14, 23, 'Setuju', 4, '2026-02-27 20:39:42', '2026-02-27 20:39:42'),
(58, 14, 24, 'Setuju', 4, '2026-02-27 20:39:42', '2026-02-27 20:39:42'),
(65, 16, 31, 'Puas', 4, '2026-02-27 20:58:54', '2026-02-27 20:58:54'),
(66, 16, 32, 'Cukup', 3, '2026-02-27 20:58:54', '2026-02-27 20:58:54'),
(67, 16, 33, 'Relevan', 1, '2026-02-27 20:58:54', '2026-02-27 20:58:54'),
(68, 16, 34, 'Cukup', 3, '2026-02-27 20:58:54', '2026-02-27 20:58:54'),
(69, 17, 20, 'Setuju', 4, '2026-02-27 21:00:14', '2026-02-27 21:00:14'),
(70, 17, 21, 'Setuju', 4, '2026-02-27 21:00:14', '2026-02-27 21:00:14'),
(71, 17, 22, 'Setuju', 4, '2026-02-27 21:00:14', '2026-02-27 21:00:14'),
(72, 17, 23, 'Setuju', 4, '2026-02-27 21:00:14', '2026-02-27 21:00:14'),
(73, 17, 24, 'Setuju', 4, '2026-02-27 21:00:14', '2026-02-27 21:00:14'),
(80, 19, 31, 'Puas', 4, '2026-02-27 21:01:53', '2026-02-27 21:01:53'),
(81, 19, 32, 'Baik', 4, '2026-02-27 21:01:53', '2026-02-27 21:01:53'),
(82, 19, 33, 'Relevan', 1, '2026-02-27 21:01:53', '2026-02-27 21:01:53'),
(83, 19, 34, 'Besar', 1, '2026-02-27 21:01:53', '2026-02-27 21:01:53');

-- --------------------------------------------------------

--
-- Table structure for table `roles`
--

CREATE TABLE `roles` (
  `id` bigint UNSIGNED NOT NULL,
  `name` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `guard_name` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

--
-- Dumping data for table `roles`
--

INSERT INTO `roles` (`id`, `name`, `guard_name`, `created_at`, `updated_at`) VALUES
(1, 'admin', 'web', '2025-12-28 12:49:06', '2025-12-28 12:49:06'),
(2, 'alumni', 'web', '2025-12-28 12:49:06', '2025-12-28 12:49:06');

-- --------------------------------------------------------

--
-- Table structure for table `role_has_permissions`
--

CREATE TABLE `role_has_permissions` (
  `permission_id` bigint UNSIGNED NOT NULL,
  `role_id` bigint UNSIGNED NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

--
-- Dumping data for table `role_has_permissions`
--

INSERT INTO `role_has_permissions` (`permission_id`, `role_id`) VALUES
(1, 1),
(2, 1),
(3, 1),
(4, 1),
(5, 1),
(6, 1),
(7, 1),
(8, 1),
(9, 1),
(10, 1),
(11, 1),
(12, 1),
(14, 1),
(15, 1),
(16, 1),
(17, 1),
(18, 1),
(19, 1),
(20, 1),
(21, 1),
(22, 1),
(23, 1),
(24, 1),
(25, 1),
(26, 1),
(28, 1),
(29, 1),
(30, 1),
(31, 1),
(32, 1),
(33, 1),
(13, 2),
(14, 2),
(15, 2),
(16, 2),
(17, 2),
(18, 2),
(19, 2),
(20, 2),
(21, 2),
(22, 2),
(23, 2),
(24, 2),
(25, 2),
(27, 2),
(28, 2),
(29, 2),
(30, 2),
(31, 2),
(32, 2);

-- --------------------------------------------------------

--
-- Table structure for table `scholarships`
--

CREATE TABLE `scholarships` (
  `id` bigint UNSIGNED NOT NULL,
  `user_id` bigint UNSIGNED NOT NULL,
  `title` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `provider` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `description` text COLLATE utf8mb4_unicode_ci NOT NULL,
  `requirements` text COLLATE utf8mb4_unicode_ci,
  `amount` varchar(255) COLLATE utf8mb4_unicode_ci DEFAULT NULL,
  `deadline` date DEFAULT NULL,
  `link` varchar(255) COLLATE utf8mb4_unicode_ci DEFAULT NULL,
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

--
-- Dumping data for table `scholarships`
--

INSERT INTO `scholarships` (`id`, `user_id`, `title`, `provider`, `description`, `requirements`, `amount`, `deadline`, `link`, `created_at`, `updated_at`) VALUES
(1, 1, 'Beasiswa Prestasi Mahasiswa TI', 'Kementerian Pendidikan, Kebudayaan, Riset, dan Teknologi', '<p><strong>Beasiswa ini ditujukan bagi mahasiswa lulusan S1 Program Studi Sistem Informasi dan Teknik Informatika yang memiliki prestasi akademik maupun non-akademik.&nbsp;</strong></p><p>Beasiswa diberikan untuk membantu biaya pendidikan dan mendukung pengembangan potensi mahasiswa selama masa studi.&nbsp;</p>', '<ul><li>Mahasiswa aktif Program Studi Sistem Informasi</li><li>Freshgraduate</li><li>IPK minimal 3.50</li><li>Memiliki surat rekomendasi dari desa</li><li>Tidak sedang menerima beasiswa lain</li></ul>', 'Rp 6.000.000 / semester', '2026-01-09', 'https://beasiswa.kemdikbud.go.id/prestasi-mahasiswa', '2025-12-28 13:16:32', '2025-12-28 13:16:32'),
(3, 1, 'Beasiswa Alumni Berprestasi', 'Yayasan Pendidikan Nusantara', '<blockquote>&nbsp;Beasiswa ini ditujukan bagi alumni berprestasi yang ingin melanjutkan studi atau mengembangkan kompetensi profesional. Beasiswa diberikan sebagai bentuk dukungan pengembangan sumber daya manusia.&nbsp;</blockquote>', '<ul><li>Alumni perguruan tinggi maksimal lulus 3 tahun terakhir</li><li>IPK minimal 3.25</li><li>Surat rekomendasi fakultas</li><li>Proposal rencana pengembangan diri</li></ul>', 'Rp 10.000.000 / semester', '2026-03-05', 'https://beasiswa.ypnusantara.or.id', '2026-01-18 03:47:05', '2026-01-18 03:49:26'),
(4, 1, 'Beasiswa Digital Talent', 'Kementerian Komunikasi dan Informatika', '<blockquote>Beasiswa pelatihan dan pengembangan kompetensi digital.&nbsp;</blockquote>', '<ul><li>Mahasiswa aktif atau alumni SI/TI</li><li>IPK minimal 3.00</li><li>Memiliki minat di bidang teknologi digital</li><li>Bersedia mengikuti seluruh rangkaian program</li></ul>', 'Rp 8.000.000', '2026-03-19', 'https://lpdp.kemenkeu.go.id', '2026-01-18 03:53:17', '2026-01-18 03:53:17');

-- --------------------------------------------------------

--
-- Table structure for table `sessions`
--

CREATE TABLE `sessions` (
  `id` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `user_id` bigint UNSIGNED DEFAULT NULL,
  `ip_address` varchar(45) COLLATE utf8mb4_unicode_ci DEFAULT NULL,
  `user_agent` text COLLATE utf8mb4_unicode_ci,
  `payload` longtext COLLATE utf8mb4_unicode_ci NOT NULL,
  `last_activity` int NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

--
-- Dumping data for table `sessions`
--

INSERT INTO `sessions` (`id`, `user_id`, `ip_address`, `user_agent`, `payload`, `last_activity`) VALUES
('l6xEI7AnFnhA9cPYzXc6ur5hOWAkGSRYmJrczzfq', NULL, '127.0.0.1', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/145.0.0.0 Safari/537.36 Edg/145.0.0.0', 'YTo0OntzOjY6Il90b2tlbiI7czo0MDoiMnVsYkFLOGNMMjl4U2pxb0VVUFBZRTFJNWxuaG9pTmpPSjVjcXJCbCI7czo2OiJfZmxhc2giO2E6Mjp7czozOiJvbGQiO2E6MDp7fXM6MzoibmV3IjthOjA6e319czo5OiJfcHJldmlvdXMiO2E6MTp7czozOiJ1cmwiO3M6NDA6Imh0dHA6Ly8xMjcuMC4wLjE6ODAwMC90cmFjZXItc3R1ZHkvbG9naW4iO31zOjM6InVybCI7YToxOntzOjg6ImludGVuZGVkIjtzOjM0OiJodHRwOi8vMTI3LjAuMC4xOjgwMDAvdHJhY2VyLXN0dWR5Ijt9fQ==', 1772537243),
('sRftpMnDPbOd7QC2WzITLjYHns5jH9h9C8G3iZiH', NULL, '127.0.0.1', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/145.0.0.0 Safari/537.36 Edg/145.0.0.0', 'YToyOntzOjY6Il90b2tlbiI7czo0MDoiR0txMmN5bFprc0haMm1qRTJ1Y0J1WXd5SXNyWFEzOWw4Z3NERjJWeiI7czo2OiJfZmxhc2giO2E6Mjp7czozOiJvbGQiO2E6MDp7fXM6MzoibmV3IjthOjA6e319fQ==', 1772533583);

-- --------------------------------------------------------

--
-- Table structure for table `users`
--

CREATE TABLE `users` (
  `id` bigint UNSIGNED NOT NULL,
  `name` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `email` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `email_verified_at` timestamp NULL DEFAULT NULL,
  `password` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  `remember_token` varchar(100) COLLATE utf8mb4_unicode_ci DEFAULT NULL,
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

--
-- Dumping data for table `users`
--

INSERT INTO `users` (`id`, `name`, `email`, `email_verified_at`, `password`, `remember_token`, `created_at`, `updated_at`) VALUES
(1, 'Administrator', 'admin@gmail.com', NULL, '$2y$12$xSO6sqCK2WczsTWxAsvofedbRlJSw0/cRpX/tSQy69WT7d5.Fazby', 'iPMNtNqnOXUQNSDL6g4kLyuFTfddMt2P1aUl0QcDobEyObgYELQbAx8bl57g', '2025-12-28 12:49:06', '2025-12-28 13:50:02'),
(12, 'Jaka', 'jaka@gmail.com', NULL, '$2y$12$e6rMhr19xrvrwPwHOvawFOSvsHdvqT2RBDfKjb9O1p5kGl9SJej9W', NULL, '2025-12-28 13:22:37', '2025-12-28 13:47:27'),
(13, 'Nadia Oktarina', 'nadiaokta847@gmail.com', NULL, '$2y$12$YsgIY.7MePyZsDKHej9UnuTJAmFJDU6oNAFr/h0lYTqW5epU1t4wK', '2raRpkCwAJsGJxqIQTYGcexmsWMpMWugLmpbh0IOnRn3mnHuZAYDiz5QfFUx', '2025-12-28 13:53:07', '2026-03-03 04:03:52'),
(14, 'Ahmad Fauzi', 'ahmad.fauzi@gmail.com', NULL, '$2y$12$FBofMveiP3.a90xnhy1skeo5aeerTRS6jWHRl2V2/hSKYeyy.jDMi', NULL, '2025-12-28 14:25:04', '2025-12-28 14:25:04'),
(15, 'Siti Aisyah', 'siti.aisyah@gmail.com', NULL, '$2y$12$dVIxRwsOxBLIXswlezm1mu7BOzea.AmHza00AuRadHm2piZ.Su.Yq', NULL, '2025-12-28 14:25:04', '2025-12-28 14:25:04'),
(16, 'Budi Santoso', 'budi.santoso@gmail.com', NULL, '$2y$12$ANofpD0fi63QiYcytgbbRuHd6ndlUR3e9vgrSNIZQ83lte.FCHYl2', NULL, '2025-12-28 14:25:05', '2025-12-28 14:25:05'),
(17, 'Rina Marlina', 'rina.marlina@gmail.com', NULL, '$2y$12$odUNr3GrWPqWg8OYA8R/0ut1ykWD0fo04SO4s0d/sd/A25ZnTTnnO', NULL, '2025-12-28 14:25:05', '2025-12-28 14:25:05'),
(18, 'Dedi Pratama', 'dedi.pratama@gmail.com', NULL, '$2y$12$iM3r22WisCptI1iywMiucuaDg7uHcOgG5DRtrv1kxULxRJofGorjy', NULL, '2025-12-28 14:25:05', '2025-12-28 14:25:05'),
(19, 'Lina Putri', 'lina.putri@gmail.com', NULL, '$2y$12$Q5ib5xtBQuT27dO3diHR2OMbvuEudjqOfQS.TykbIuoiVPMvg/AD6', NULL, '2025-12-28 14:25:06', '2025-12-28 14:25:06'),
(20, 'Rizky Maulana', 'rizky.maulana@gmail.com', NULL, '$2y$12$727wKDdO61RcfK5i5AdKv.HqYqmpnd/byVOOIa6L./eGMgFkBcCha', NULL, '2025-12-28 14:25:06', '2025-12-28 14:25:06'),
(21, 'Putri Ananda', 'putri.ananda@gmail.com', NULL, '$2y$12$8P2oUga1mv3Pau2FdjROmexrir9PpZio/9e13rwx1bKCmZXbaCSNy', NULL, '2025-12-28 14:25:07', '2025-12-28 14:25:07'),
(22, 'Andi Wijaya', 'andi.wijaya@gmail.com', NULL, '$2y$12$xiiauIYOnswDUkugw655tOCVXfcaRppT8lOHJTOCiE7MQ14vTaDUa', NULL, '2025-12-28 14:25:07', '2025-12-28 14:25:07'),
(23, 'Nurul Hidayah', 'nurul.hidayah@gmail.com', NULL, '$2y$12$sdEC9rUCkiQmUkq28kOcf.u7wmevQOTYq75FXWXJ.ERYjYxN.qR.C', NULL, '2025-12-28 14:25:07', '2025-12-28 14:25:07'),
(50, 'Dedi Pratama', 'dedi.pratama20@gmail.com', NULL, '$2y$12$HuaXSxIQcoAxOJR8tqgbveMVNl9EkxKed6G6QatIr36CwhYo4ML8e', NULL, '2026-01-13 02:44:37', '2026-01-13 04:23:40'),
(68, 'Ahmad Fauzi', 'ahmad.fauzi02@gmail.com', NULL, '$2y$12$L6h8w6jY/0lq7Rg9pVNCieoAdWeDnTXArpJyWVmF0U5SePkEmTshm', NULL, '2026-01-13 03:06:32', '2026-01-13 03:06:32'),
(69, 'Siti Aminah', 'siti.aminah@gmail.com', NULL, '$2y$12$opetj9YrhvwZrKYlkNgpkuVKvvkwuapvISFtaS3dAO1gcInlst53.', NULL, '2026-01-13 03:06:32', '2026-01-13 03:06:32'),
(70, 'Budi Santoso', 'budi.santoso22@gmail.com', NULL, '$2y$12$btj7XzZQL/KunfJfTPAk1uvRBN3uUbz5bBgMQrCllBm1wvk/.gWcu', NULL, '2026-01-13 03:06:33', '2026-01-13 03:06:33'),
(71, 'Rina Putri', 'rina.putri@gmail.com', NULL, '$2y$12$wzenw3s2S9B9MzI.nNGsk.j3I.TypVBnnJYAJdKmLC3LO2lDYm5oa', NULL, '2026-01-13 03:06:34', '2026-01-13 03:06:34'),
(72, 'M syarifuddin', 'udin@gmail.com', NULL, '$2y$12$ctoSJBmL7qa8yOJ.o4ECJu7kdCgZa39d/uTfcHNbvK7WX4N5/wZbW', NULL, '2026-01-14 08:09:07', '2026-01-14 08:09:07'),
(78, 'Ahmad Fairuz', 'ahmadfairuz1@gmail.com', NULL, '$2y$12$xOxJB2QNwYwIaqIuJyyeyOh42YslIFFuAm7lQFgzZf6vA07JiXQYy', NULL, '2026-01-26 01:00:24', '2026-01-27 02:29:18'),
(80, 'Mustofa', 'mustop4@gmail.com', NULL, '$2y$12$yDNh98AGPvSRBBjkAf8Hg.SXVYzSL7z8xB7Zh1GMwAMuFiDCYyYkG', NULL, '2026-02-27 20:13:10', '2026-02-27 20:13:10'),
(81, 'Ikbal', 'ikbal2@gmail.com', NULL, '$2y$12$es8n5pV/7HQdhl8OUnaI.OAXN7aOo1/Hg/UARfwnQQa8MbeXwzIty', NULL, '2026-02-27 21:22:01', '2026-02-27 21:22:01'),
(82, 'Siti Rohaya', 'sitiryh@gmail.com', NULL, '$2y$12$.rH1CXHxJcwBi55V1VFCfe8f2EMHZOx2OYbUGeanKd9dxhq5LIkwS', NULL, '2026-02-27 21:22:01', '2026-02-27 21:22:01'),
(83, 'Budiman', 'budiman02@gmail.com', NULL, '$2y$12$XkVNmCJrWcLb16GJ0NwAIeq1tLb/ru3tP9wfN24T4VBH0J/Vt2SlG', NULL, '2026-02-27 21:22:02', '2026-02-27 21:22:02'),
(84, 'Rina badria', 'rinabadr@gmail.com', NULL, '$2y$12$rDCeV1V1gLfteNXbEePleu6QaU1Sl3viI52l.Xc1ZaZ0OH5QT17l.', NULL, '2026-02-27 21:22:02', '2026-02-27 21:22:02'),
(85, 'Rendi Pratama', 'rendipratama10@gmail.com', NULL, '$2y$12$LDJEnhHBMELnMw6PuZH59unl5nUxL36SLkI1nMmTmlUpNrV2AyrJO', NULL, '2026-02-27 21:36:09', '2026-02-27 21:36:09');

--
-- Indexes for dumped tables
--

--
-- Indexes for table `alumnis`
--
ALTER TABLE `alumnis`
  ADD PRIMARY KEY (`id`),
  ADD UNIQUE KEY `alumnis_nim_unique` (`nim`),
  ADD UNIQUE KEY `alumnis_email_unique` (`email`),
  ADD KEY `alumnis_user_id_foreign` (`user_id`);

--
-- Indexes for table `cache`
--
ALTER TABLE `cache`
  ADD PRIMARY KEY (`key`);

--
-- Indexes for table `cache_locks`
--
ALTER TABLE `cache_locks`
  ADD PRIMARY KEY (`key`);

--
-- Indexes for table `exports`
--
ALTER TABLE `exports`
  ADD PRIMARY KEY (`id`),
  ADD KEY `exports_user_id_foreign` (`user_id`);

--
-- Indexes for table `failed_import_rows`
--
ALTER TABLE `failed_import_rows`
  ADD PRIMARY KEY (`id`),
  ADD KEY `failed_import_rows_import_id_foreign` (`import_id`);

--
-- Indexes for table `failed_jobs`
--
ALTER TABLE `failed_jobs`
  ADD PRIMARY KEY (`id`),
  ADD UNIQUE KEY `failed_jobs_uuid_unique` (`uuid`);

--
-- Indexes for table `forum_replies`
--
ALTER TABLE `forum_replies`
  ADD PRIMARY KEY (`id`),
  ADD KEY `forum_replies_user_id_foreign` (`user_id`),
  ADD KEY `forum_replies_topic_id_index` (`topic_id`);

--
-- Indexes for table `forum_topics`
--
ALTER TABLE `forum_topics`
  ADD PRIMARY KEY (`id`),
  ADD KEY `forum_topics_user_id_foreign` (`user_id`),
  ADD KEY `forum_topics_created_at_is_pinned_index` (`created_at`,`is_pinned`);

--
-- Indexes for table `imports`
--
ALTER TABLE `imports`
  ADD PRIMARY KEY (`id`),
  ADD KEY `imports_user_id_foreign` (`user_id`);

--
-- Indexes for table `internships`
--
ALTER TABLE `internships`
  ADD PRIMARY KEY (`id`),
  ADD KEY `internships_user_id_foreign` (`user_id`);

--
-- Indexes for table `jobs`
--
ALTER TABLE `jobs`
  ADD PRIMARY KEY (`id`),
  ADD KEY `jobs_queue_index` (`queue`);

--
-- Indexes for table `job_batches`
--
ALTER TABLE `job_batches`
  ADD PRIMARY KEY (`id`);

--
-- Indexes for table `job_opportunities`
--
ALTER TABLE `job_opportunities`
  ADD PRIMARY KEY (`id`),
  ADD KEY `job_opportunities_user_id_foreign` (`user_id`);

--
-- Indexes for table `migrations`
--
ALTER TABLE `migrations`
  ADD PRIMARY KEY (`id`);

--
-- Indexes for table `model_has_permissions`
--
ALTER TABLE `model_has_permissions`
  ADD PRIMARY KEY (`permission_id`,`model_id`,`model_type`),
  ADD KEY `model_has_permissions_model_id_model_type_index` (`model_id`,`model_type`);

--
-- Indexes for table `model_has_roles`
--
ALTER TABLE `model_has_roles`
  ADD PRIMARY KEY (`role_id`,`model_id`,`model_type`),
  ADD KEY `model_has_roles_model_id_model_type_index` (`model_id`,`model_type`);

--
-- Indexes for table `notifications`
--
ALTER TABLE `notifications`
  ADD PRIMARY KEY (`id`),
  ADD KEY `notifications_notifiable_type_notifiable_id_index` (`notifiable_type`,`notifiable_id`);

--
-- Indexes for table `password_reset_tokens`
--
ALTER TABLE `password_reset_tokens`
  ADD PRIMARY KEY (`email`);

--
-- Indexes for table `permissions`
--
ALTER TABLE `permissions`
  ADD PRIMARY KEY (`id`),
  ADD UNIQUE KEY `permissions_name_guard_name_unique` (`name`,`guard_name`);

--
-- Indexes for table `questionnaires`
--
ALTER TABLE `questionnaires`
  ADD PRIMARY KEY (`id`);

--
-- Indexes for table `questionnaire_questions`
--
ALTER TABLE `questionnaire_questions`
  ADD PRIMARY KEY (`id`),
  ADD KEY `questionnaire_questions_questionnaire_id_foreign` (`questionnaire_id`);

--
-- Indexes for table `questionnaire_responses`
--
ALTER TABLE `questionnaire_responses`
  ADD PRIMARY KEY (`id`),
  ADD UNIQUE KEY `questionnaire_responses_questionnaire_id_user_id_unique` (`questionnaire_id`,`user_id`),
  ADD KEY `questionnaire_responses_user_id_foreign` (`user_id`);

--
-- Indexes for table `response_answers`
--
ALTER TABLE `response_answers`
  ADD PRIMARY KEY (`id`),
  ADD KEY `response_answers_response_id_foreign` (`response_id`),
  ADD KEY `response_answers_question_id_foreign` (`question_id`);

--
-- Indexes for table `roles`
--
ALTER TABLE `roles`
  ADD PRIMARY KEY (`id`),
  ADD UNIQUE KEY `roles_name_guard_name_unique` (`name`,`guard_name`);

--
-- Indexes for table `role_has_permissions`
--
ALTER TABLE `role_has_permissions`
  ADD PRIMARY KEY (`permission_id`,`role_id`),
  ADD KEY `role_has_permissions_role_id_foreign` (`role_id`);

--
-- Indexes for table `scholarships`
--
ALTER TABLE `scholarships`
  ADD PRIMARY KEY (`id`),
  ADD KEY `scholarships_user_id_foreign` (`user_id`);

--
-- Indexes for table `sessions`
--
ALTER TABLE `sessions`
  ADD PRIMARY KEY (`id`),
  ADD KEY `sessions_user_id_index` (`user_id`),
  ADD KEY `sessions_last_activity_index` (`last_activity`);

--
-- Indexes for table `users`
--
ALTER TABLE `users`
  ADD PRIMARY KEY (`id`),
  ADD UNIQUE KEY `users_email_unique` (`email`);

--
-- AUTO_INCREMENT for dumped tables
--

--
-- AUTO_INCREMENT for table `alumnis`
--
ALTER TABLE `alumnis`
  MODIFY `id` bigint UNSIGNED NOT NULL AUTO_INCREMENT, AUTO_INCREMENT=124;

--
-- AUTO_INCREMENT for table `exports`
--
ALTER TABLE `exports`
  MODIFY `id` bigint UNSIGNED NOT NULL AUTO_INCREMENT, AUTO_INCREMENT=7;

--
-- AUTO_INCREMENT for table `failed_import_rows`
--
ALTER TABLE `failed_import_rows`
  MODIFY `id` bigint UNSIGNED NOT NULL AUTO_INCREMENT;

--
-- AUTO_INCREMENT for table `failed_jobs`
--
ALTER TABLE `failed_jobs`
  MODIFY `id` bigint UNSIGNED NOT NULL AUTO_INCREMENT;

--
-- AUTO_INCREMENT for table `forum_replies`
--
ALTER TABLE `forum_replies`
  MODIFY `id` bigint UNSIGNED NOT NULL AUTO_INCREMENT, AUTO_INCREMENT=15;

--
-- AUTO_INCREMENT for table `forum_topics`
--
ALTER TABLE `forum_topics`
  MODIFY `id` bigint UNSIGNED NOT NULL AUTO_INCREMENT, AUTO_INCREMENT=7;

--
-- AUTO_INCREMENT for table `imports`
--
ALTER TABLE `imports`
  MODIFY `id` bigint UNSIGNED NOT NULL AUTO_INCREMENT, AUTO_INCREMENT=8;

--
-- AUTO_INCREMENT for table `internships`
--
ALTER TABLE `internships`
  MODIFY `id` bigint UNSIGNED NOT NULL AUTO_INCREMENT, AUTO_INCREMENT=6;

--
-- AUTO_INCREMENT for table `jobs`
--
ALTER TABLE `jobs`
  MODIFY `id` bigint UNSIGNED NOT NULL AUTO_INCREMENT, AUTO_INCREMENT=7;

--
-- AUTO_INCREMENT for table `job_opportunities`
--
ALTER TABLE `job_opportunities`
  MODIFY `id` bigint UNSIGNED NOT NULL AUTO_INCREMENT, AUTO_INCREMENT=8;

--
-- AUTO_INCREMENT for table `migrations`
--
ALTER TABLE `migrations`
  MODIFY `id` int UNSIGNED NOT NULL AUTO_INCREMENT, AUTO_INCREMENT=55;

--
-- AUTO_INCREMENT for table `permissions`
--
ALTER TABLE `permissions`
  MODIFY `id` bigint UNSIGNED NOT NULL AUTO_INCREMENT, AUTO_INCREMENT=34;

--
-- AUTO_INCREMENT for table `questionnaires`
--
ALTER TABLE `questionnaires`
  MODIFY `id` bigint UNSIGNED NOT NULL AUTO_INCREMENT, AUTO_INCREMENT=14;

--
-- AUTO_INCREMENT for table `questionnaire_questions`
--
ALTER TABLE `questionnaire_questions`
  MODIFY `id` bigint UNSIGNED NOT NULL AUTO_INCREMENT, AUTO_INCREMENT=36;

--
-- AUTO_INCREMENT for table `questionnaire_responses`
--
ALTER TABLE `questionnaire_responses`
  MODIFY `id` bigint UNSIGNED NOT NULL AUTO_INCREMENT, AUTO_INCREMENT=20;

--
-- AUTO_INCREMENT for table `response_answers`
--
ALTER TABLE `response_answers`
  MODIFY `id` bigint UNSIGNED NOT NULL AUTO_INCREMENT, AUTO_INCREMENT=84;

--
-- AUTO_INCREMENT for table `roles`
--
ALTER TABLE `roles`
  MODIFY `id` bigint UNSIGNED NOT NULL AUTO_INCREMENT, AUTO_INCREMENT=3;

--
-- AUTO_INCREMENT for table `scholarships`
--
ALTER TABLE `scholarships`
  MODIFY `id` bigint UNSIGNED NOT NULL AUTO_INCREMENT, AUTO_INCREMENT=8;

--
-- AUTO_INCREMENT for table `users`
--
ALTER TABLE `users`
  MODIFY `id` bigint UNSIGNED NOT NULL AUTO_INCREMENT, AUTO_INCREMENT=86;

--
-- Constraints for dumped tables
--

--
-- Constraints for table `alumnis`
--
ALTER TABLE `alumnis`
  ADD CONSTRAINT `alumnis_user_id_foreign` FOREIGN KEY (`user_id`) REFERENCES `users` (`id`) ON DELETE CASCADE;

--
-- Constraints for table `exports`
--
ALTER TABLE `exports`
  ADD CONSTRAINT `exports_user_id_foreign` FOREIGN KEY (`user_id`) REFERENCES `users` (`id`) ON DELETE CASCADE;

--
-- Constraints for table `failed_import_rows`
--
ALTER TABLE `failed_import_rows`
  ADD CONSTRAINT `failed_import_rows_import_id_foreign` FOREIGN KEY (`import_id`) REFERENCES `imports` (`id`) ON DELETE CASCADE;

--
-- Constraints for table `forum_replies`
--
ALTER TABLE `forum_replies`
  ADD CONSTRAINT `forum_replies_topic_id_foreign` FOREIGN KEY (`topic_id`) REFERENCES `forum_topics` (`id`) ON DELETE CASCADE,
  ADD CONSTRAINT `forum_replies_user_id_foreign` FOREIGN KEY (`user_id`) REFERENCES `users` (`id`) ON DELETE CASCADE;

--
-- Constraints for table `forum_topics`
--
ALTER TABLE `forum_topics`
  ADD CONSTRAINT `forum_topics_user_id_foreign` FOREIGN KEY (`user_id`) REFERENCES `users` (`id`) ON DELETE CASCADE;

--
-- Constraints for table `imports`
--
ALTER TABLE `imports`
  ADD CONSTRAINT `imports_user_id_foreign` FOREIGN KEY (`user_id`) REFERENCES `users` (`id`) ON DELETE CASCADE;

--
-- Constraints for table `internships`
--
ALTER TABLE `internships`
  ADD CONSTRAINT `internships_user_id_foreign` FOREIGN KEY (`user_id`) REFERENCES `users` (`id`) ON DELETE CASCADE;

--
-- Constraints for table `job_opportunities`
--
ALTER TABLE `job_opportunities`
  ADD CONSTRAINT `job_opportunities_user_id_foreign` FOREIGN KEY (`user_id`) REFERENCES `users` (`id`) ON DELETE CASCADE;

--
-- Constraints for table `model_has_permissions`
--
ALTER TABLE `model_has_permissions`
  ADD CONSTRAINT `model_has_permissions_permission_id_foreign` FOREIGN KEY (`permission_id`) REFERENCES `permissions` (`id`) ON DELETE CASCADE;

--
-- Constraints for table `model_has_roles`
--
ALTER TABLE `model_has_roles`
  ADD CONSTRAINT `model_has_roles_role_id_foreign` FOREIGN KEY (`role_id`) REFERENCES `roles` (`id`) ON DELETE CASCADE;

--
-- Constraints for table `questionnaire_questions`
--
ALTER TABLE `questionnaire_questions`
  ADD CONSTRAINT `questionnaire_questions_questionnaire_id_foreign` FOREIGN KEY (`questionnaire_id`) REFERENCES `questionnaires` (`id`) ON DELETE CASCADE;

--
-- Constraints for table `questionnaire_responses`
--
ALTER TABLE `questionnaire_responses`
  ADD CONSTRAINT `questionnaire_responses_questionnaire_id_foreign` FOREIGN KEY (`questionnaire_id`) REFERENCES `questionnaires` (`id`) ON DELETE CASCADE,
  ADD CONSTRAINT `questionnaire_responses_user_id_foreign` FOREIGN KEY (`user_id`) REFERENCES `users` (`id`) ON DELETE CASCADE;

--
-- Constraints for table `response_answers`
--
ALTER TABLE `response_answers`
  ADD CONSTRAINT `response_answers_question_id_foreign` FOREIGN KEY (`question_id`) REFERENCES `questionnaire_questions` (`id`) ON DELETE CASCADE,
  ADD CONSTRAINT `response_answers_response_id_foreign` FOREIGN KEY (`response_id`) REFERENCES `questionnaire_responses` (`id`) ON DELETE CASCADE;

--
-- Constraints for table `role_has_permissions`
--
ALTER TABLE `role_has_permissions`
  ADD CONSTRAINT `role_has_permissions_permission_id_foreign` FOREIGN KEY (`permission_id`) REFERENCES `permissions` (`id`) ON DELETE CASCADE,
  ADD CONSTRAINT `role_has_permissions_role_id_foreign` FOREIGN KEY (`role_id`) REFERENCES `roles` (`id`) ON DELETE CASCADE;

--
-- Constraints for table `scholarships`
--
ALTER TABLE `scholarships`
  ADD CONSTRAINT `scholarships_user_id_foreign` FOREIGN KEY (`user_id`) REFERENCES `users` (`id`) ON DELETE CASCADE;
COMMIT;

/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */;
/*!40101 SET CHARACTER_SET_RESULTS=@OLD_CHARACTER_SET_RESULTS */;
/*!40101 SET COLLATION_CONNECTION=@OLD_COLLATION_CONNECTION */;
