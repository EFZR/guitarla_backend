DELIMITER //

CREATE PROCEDURE kill_process_by_database_user(
    IN database_name VARCHAR(100),
    IN user_name VARCHAR(100)
)
BEGIN
    DECLARE done INT DEFAULT FALSE;
    DECLARE process_id INT;
    DECLARE cur CURSOR FOR
        SELECT id
        FROM information_schema.`PROCESSLIST`
        WHERE DB = database_name OR USER = user_name;
    DECLARE CONTINUE HANDLER FOR NOT FOUND SET done = TRUE;

    OPEN cur;
    read_loop: LOOP
        FETCH cur INTO process_id;
        IF done THEN
            LEAVE read_loop;
        END IF;
        SET @kill_query = CONCAT('KILL ', process_id);
        PREPARE stmt FROM @kill_query;
        EXECUTE stmt;
        DEALLOCATE PREPARE stmt;
    END LOOP;
    CLOSE cur;
END//

DELIMITER ;


CREATE TABLE guitarras (
	-- pk
	`id` INT AUTO_INCREMENT,
	
	-- properties
	`name` VARCHAR(45) NOT NULL,
	`img` VARCHAR(100),
	`description` VARCHAR(250) NOT NULL,
	`price` DECIMAL(10,2) NOT NULL,
	
	-- timestamps
	`cid` INT NOT NULL,
	`ctime` TIMESTAMP NOT NULL,
	`mid` INT NOT NULL,
	`mtime` TIMESTAMP NOT NULL,
	
	PRIMARY KEY (id)
);