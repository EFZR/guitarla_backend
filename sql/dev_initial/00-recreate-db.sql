-- DEV ONLY - Brute Force DROP DB (for local dev and unit test)
-- CALL ("database", "user")
CALL kill_process_by_database_user ("inventario_guitarras", "app_user");

-- DEV ONLY - Drop DB and User
DROP DATABASE IF EXISTS `inventario_guitarras`;
DROP USER IF EXISTS 'app_user'@'localhost';

-- DEV ONLY - Dev only password (for local dev and unit test)
CREATE USER 'app_user'@'localhost' IDENTIFIED BY 'dev_only_pwd';

CREATE DATABASE `inventario_guitarras`;