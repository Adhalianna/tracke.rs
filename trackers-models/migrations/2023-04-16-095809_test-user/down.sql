-- This file should undo anything in `up.sql`
DELETE FROM tasks WHERE tracker_id IN (
  SELECT tracker_id FROM trackers
  WHERE user_id = '00000000-0000-0000-0000-000000000000'
);
DELETE FROM trackers WHERE user_id = '00000000-0000-0000-0000-000000000000';
DELETE FROM users WHERE user_id = '00000000-0000-0000-0000-000000000000'; 