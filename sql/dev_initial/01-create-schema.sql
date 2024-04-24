CREATE TABLE guitarras (
	-- pk
	`id` INT AUTO_INCREMENT,
	
	-- properties
	`name` VARCHAR(45) NOT NULL,
	`description` VARCHAR(250) NOT NULL,
	`price` DECIMAL(10,2) NOT NULL,
	`img` VARCHAR(100),
	
	-- timestamps
	`cid` INT NOT NULL,
	`ctime` TIMESTAMP NOT NULL,
	`mid` INT NOT NULL,
	`mtime` TIMESTAMP NOT NULL,
	
	PRIMARY KEY (id)
)