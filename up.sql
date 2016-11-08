create table events
(id serial primary key,
 event text);

create function notify_trigger() returns trigger as $$
declare
begin
    if TG_OP = 'INSERT' or TG_OP = 'UPDATE' then
        execute 'NOTIFY '
        || TG_TABLE_NAME
        || ', '''
        || TG_OP
        || ' '
        || NEW
        || '''';
    else
        execute 'NOTIFY '
        || TG_TABLE_NAME
        || ', '''
        || TG_OP
        || '''';
    end if;
    return new;
end;
$$ LANGUAGE plpgsql;

create trigger event_trigger
after insert or update or delete on events
for each row execute procedure notify_trigger();

