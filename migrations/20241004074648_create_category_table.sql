-- Add migration script here

CREATE OR REPLACE TABLE `category_table` (
	`id` INTEGER NOT NULL AUTO_INCREMENT UNIQUE,
	`cat_id` VARCHAR(255) NOT NULL,
	`cat_name` VARCHAR(50) NOT NULL,
	`drive_type` VARCHAR(50) NOT NULL,
	`cat_status` TINYINT NOT NULL DEFAULT 0,
	`ctime` TIMESTAMP NOT NULL DEFAULT current_timestamp(),
	`mtime` TIMESTAMP NOT NULL DEFAULT current_timestamp(),
	PRIMARY KEY(`id`)
);

