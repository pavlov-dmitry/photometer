DELETE FROM group_feed
WHERE id IN (
    SELECT gid FROM (
        SELECT DISTINCT g.id as gid FROM group_feed as g
        LEFT JOIN scheduled_events as e ON( e.id = g.scheduled_id )
        WHERE e.event_id IN ( 1, 4 ) AND g.state='start'
    ) as gg
)
