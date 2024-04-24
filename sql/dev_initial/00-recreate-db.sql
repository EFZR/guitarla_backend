-- DEV ONLY - Drop DB and User
DROP DATABASE IF EXISTS `inventario_guitarras`;
DROP USER IF EXISTS 'app_user'@'localhost';

-- DEV ONLY - Dev only password (for local dev and unit test)
CREATE DATABASE `inventario_guitarras`;
CREATE USER 'app_user'@'localhost' IDENTIFIED BY 'dev_only_pwd';
GRANT ALL PRIVILEGES ON `inventario_guitarras`.* TO 'app_user'@'localhost'