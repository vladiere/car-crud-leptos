-- Add migration script here

CREATE OR REPLACE TABLE `color_table` (
	`id` INTEGER NOT NULL AUTO_INCREMENT UNIQUE,
	`color_id` VARCHAR(255) NOT NULL,
	`color_name` VARCHAR(50) NOT NULL,
	`color_type` VARCHAR(50) NOT NULL,
	`color_status` TINYINT NOT NULL DEFAULT 0,
	`ctime` TIMESTAMP NOT NULL DEFAULT current_timestamp(),
	`mtime` TIMESTAMP NOT NULL DEFAULT current_timestamp(),
	PRIMARY KEY(`id`)
);

