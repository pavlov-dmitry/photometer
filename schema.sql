CREATE TABLE `users` (
    `id` bigint(20) NOT NULL AUTO_INCREMENT,
    `login` varchar(16) NOT NULL DEFAULT '',
    `password` varchar(32) NOT NULL DEFAULT '',
    PRIMARY KEY (`id`),
    KEY `login_idx` (`login`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;

CREATE TABLE `images` (
    `id` bigint(20) NOT NULL AUTO_INCREMENT,
    `owner_id` int(4) unsigned DEFAULT '0',
    `upload_time` int(11) NOT NULL DEFAULT '0',
    `type` enum( 'jpg', 'png' ) NOT NULL DEFAULT 'jpg',
    `width` int(4) unsigned DEFAULT '0',
    `height` int(4) unsigned DEFAULT '0',
    `name` varchar(64) NOT NULL DEFAULT '',
    `iso` int(11) unsigned DEFAULT '0',
    `shutter_speed` int(11) DEFAULT '0',
    `aperture` decimal(8,4) NOT NULL DEFAULT '0',
    `focal_length` int(4) unsigned DEFAULT '0',
    `focal_length_35mm` int(4) unsigned DEFAULT '0',
    `camera_model` varchar(64) NOT NULL DEFAULT '',
    PRIMARY KEY ( `id` ),
    KEY `owner_image` ( `owner_id`, `upload_time` )
) ENGINE=InnoDB DEFAULT CHARSET=utf8;
