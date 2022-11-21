# Feature

- Feature Name: core-reservation
- Start Date: 2022-11-20

## Summary

a core reservation service that solves the problem of reserving a resource for a period of time. we leverage Postgres EXCLUDE constraints to ensure that only one reservation can be made for a given time.

## Motivation

We need a common solution for various reservation requirements: 1) calendar booking; 2)home booking; 3)meeting room booking; 4)etc.


## Guide-level explanation


### Service interface

We would use gRPC as a service interface. Below is the proto definition:

```proto
enum ReservationStatus {
    UNKNOWN = 0;
    PENDING = 1;
    CONFIRMED = 2;
    BLOCKED = 3;
}

enum ReservationUpdateType {
    UNKNOWN = 0;
    CREATE = 1;
    UPDATE = 2;
    DELETE = 3;
}

message Reservation {
    string id = 1;
    string user_id = 2;
    ReservationStatus status= 3;
    // resource reservation window
    string resource_id = 4;
    google.protobuf.Timestamp start =5;
    google.protobuf.Timestamp end = 6;
    // extra note
    string note = 7;
}

message ReserveRequest {
    Reservation reservation = 1;
}

message ReserveResponse {
    Reservation reservation = 1;
}

message UpdateRequest {
    string note = 1;
}

message UpdateResponse {
    Reservation reservation = 1;
}

message ConfirmRequest {
    string id = 1;
}

message ConfirmResponse {
    Reservation reservation = 1;
}

message CancelRequest {
    string id = 1;
}

message CancelResponse {
    Reservation reservation = 1;
}

message GetRequest {
    string id = 1;
    ReservationStatus  status = 2;
    string resource_id = 3;
    uint64 page = 4;
    uint64 size = 5;
}

message GetResponse {
    Reservation reservation = 1;
}

message QueryRequest {
    string id = 1;
}

message QueryResponse {
    uint64 total  = 1;
    repeated Reservation data;
}

message ListenRequest {}
message ListenResponse {

}
service ReservationService {
    rpc reserve(ReserveRequest) returns (ReserveResponse);
    rpc update(UpdateRequest) returns (UpdateRequest);
    rpc confirm(ConfirmRequest) returns (ConfirmResponse);
    rpc cancel(CancelRequest) returns (CancelResponse);
    rpc get(GetRequest) returns (GetResponse);
    rpc query(QueryRequest) returns (QueryResponse);
    // another system can monitor newly added/confirmed/cancelled reservation
    rpc listen(ListenRequest) returns (ListenResponse);
}
```

### Database schema

We use Postgres as the database. Below is the schema:

```sql
CREATE SCHEMA rsvp;
CREATE TYPE rsvp.reservation_status as ENUM('unknown','pending','confirmed','blocked');
CREATE TYPE rsvp.reservation_update_type as ENUM('unknown','create','update','delete');

CREATE TABLE reservation (
    id uuid PRIMARY EKY,
    --  user_id maybe is int or string, we use string.
    user_id varchar(64) NOT NULL,
    status rsvp.reservation_status NOT NULL DEFAULT 'pending',
    resource_id varchar(64) NOT NULL,
    timespan tstzrange NOT NULL,
    note text,

    CONSTRAINT reservation_pkey PRIMARY KEY (id),
    -- https://www.postgresql.org/docs/current/indexes-types.html#INDEXES-TYPE-GIST
    CONSTRAINT reservation_conflict EXCLUDE USING gist (resource_id WITH = ,timespan WITH &&)
);

CREATE TABLE reservation_update (
    id SERIAL NOT NULL,
    reservation_id uuid NOT NULL,
    op rsvp.reservation_update_type NOT NULL
)
CREATE OR REPLACE FUNCTION rsvp.query(user_id text,resource_id text,during ) RETURNS TABLE AS
$$
BEGIN

END
$$

CREATE OR REPLACE FUNCTION rsvp.reservation_trigger() RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'INSERT' THEN
        INSERT INTO rsvp.reservation_change (reservation_id,op) VALUES (NEW.id,'create');
    ELSIF TG_OP = 'UPDATE' THEN
        IF OLD.status <> NEW.status THEN
            INSERT INTO rsvp.reservation_change (reservation_id,op) VALUES (NEW.id,'update');
    ELSIF TG_OP = 'DELETE' THEN
            INSERT INTO rsvp.reservation_change (reservation_id,op) VALUES (NEW.id,'delete');
    END IF;
    NOTIFY reservation_update;
    RETURNS NULL;
END;
$$ LANGUAGE plpgsql;
CREATE TRIGGER reservation_trigger AFTER INSERT OR UPDATE OR DELETE ON rsvp.reservation FOR EACH  ROW EXCLUDE PROCEDURE  rsvp.reservation_trigger();
```

## Reference-level explanation

## Drawbacks

N/A

## Rationale and alternatives

## Prior art

## Unresolved questions

## Future possibilities
