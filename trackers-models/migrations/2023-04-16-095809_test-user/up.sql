-- Your SQL goes here
INSERT INTO users VALUES 
-- password: password$123
('00000000-0000-0000-0000-000000000000', 'test@tracke.rs', '$2a$12$EZbyhEM/VbrWhzmOs9UAteRiJWYygryQC0cSZhDq5aKHIE56WmZby'::bytea);

INSERT INTO trackers VALUES
('00000000-0000-0000-0000-000000000000', '00000000-0000-0000-0000-000000000000', 'Backlog', true),
('00000000-0000-0000-0000-000000000001', '00000000-0000-0000-0000-000000000000', 'Household Chores', false);

INSERT INTO tasks VALUES
('00000000-0000-0000-0000-000000000000', '00000000-0000-0000-0000-000000000001', null, 'Do laundry', 'black + grey colored', 60*60, null, null, ARRAY ['chore']),
('00000000-0000-0000-0000-000000000001', '00000000-0000-0000-0000-000000000000', now(), 'Do CyberSec assignment', null, 150*60, now(), null, ARRAY ['studies', 'CyberSec', 'assignments']),
('00000000-0000-0000-0000-000000000002', '00000000-0000-0000-0000-000000000000', null, 'Study for Web Apps test', 'read the flask docs, review REST API guidelines, practice manipulating DOM with JS, review CSS properties used during classes', 240*60, now(), null, ARRAY ['studies', 'Web Apps']),
('00000000-0000-0000-0000-000000000003', '00000000-0000-0000-0000-000000000001', null, 'Vacuum clean the bedroom', null, 8*60 + 40, null, null, ARRAY ['chore'])
;