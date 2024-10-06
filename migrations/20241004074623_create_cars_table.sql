-- Add migration script here
CREATE OR REPLACE TABLE `car_table` (
	`id` INTEGER NOT NULL AUTO_INCREMENT UNIQUE PRIMARY KEY,
	`car_id` VARCHAR(255) NOT NULL,
	`brand` VARCHAR(25) NOT NULL,
	`cat_id` INTEGER NOT NULL,
	`color_id` INTEGER NOT NULL,
	`model` VARCHAR(255) NOT NULL,
	`year` SMALLINT NOT NULL,
	`fuel_type` VARCHAR(255) NOT NULL,
	`engine_size` SMALLINT NOT NULL,
	`transmission_type` VARCHAR(255) NOT NULL,
	`vin` VARCHAR(50) NOT NULL,
	`ctime` TIMESTAMP NOT NULL DEFAULT current_timestamp(),
	`mtime` TIMESTAMP NOT NULL DEFAULT current_timestamp()
);
