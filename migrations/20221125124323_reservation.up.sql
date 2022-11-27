-- Add up migration script here
CREATE TYPE rsvp.reservation_status as ENUM('unknown', 'pending', 'confirmed', 'blocked');
CREATE TYPE rsvp.reservation_update_type as ENUM('unknown', 'create', 'update', 'delete');

CREATE TABLE rsvp.reservation (
    id uuid NOT NULL DEFAULT gen_random_uuid(),
    --  user_id maybe is int or string, we use string.
    user_id varchar(64) NOT NULL,
    status rsvp.reservation_status NOT NULL DEFAULT 'pending',
    resource_id varchar(64) NOT NULL,
    timespan tstzrange NOT NULL,
    note text,
    CONSTRAINT reservation_pkey PRIMARY KEY (id),
    -- https://www.postgresql.org/docs/current/indexes-types.html#INDEXES-TYPE-GIST
    CONSTRAINT reservation_conflict EXCLUDE USING gist (resource_id WITH =, timespan WITH &&)
);

CREATE INDEX reservation_resource_id_index ON rsvp.reservation (resource_id);
CREATE INDEX reservation_user_id_index ON rsvp.reservation (user_id);
