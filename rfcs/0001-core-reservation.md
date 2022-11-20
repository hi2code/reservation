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

service ReservationService {
    rpc reserve(ReserveRequest) returns (ReserveResponse);
    rpc update(UpdateRequest) returns (UpdateRequest);
    rpc confirm(ConfirmRequest) returns (ConfirmResponse);
    rpc cancel(CancelRequest) returns (CancelResponse);
    rpc get(GetRequest) returns (GetResponse);
    rpc query(QueryRequest) returns (QueryResponse);

    rpc listen() returns
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

CREATE TABLE reservation (
    id SERIAL NOT NULL,
    reservation_id uuid NOT NULL,
    op rsvp.reservation_update_type NOT NULL
)
CREATE OR REPLACE FUNCTION rsvp.reservation_trigger() RETURNS TRIGGER AS
$$
BEGIN
END
$$

CREATE TRIGGER reservation_trigger AFTER INSERT OR UPDATE OR DELETE ON rsvp.reservation FOR
```

## Reference-level explanation

## Drawbacks

Why should we *not* do this?

## Rationale and alternatives

## Prior art

## Unresolved questions

## Future possibilities
