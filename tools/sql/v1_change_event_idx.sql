ALTER TABLE `scheduled_events` DROP INDEX `time_idx`;
ALTER TABLE `scheduled_events` ADD INDEX `state_evt_idx` ( `state`, `event_id` );
