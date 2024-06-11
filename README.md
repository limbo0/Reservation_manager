## Notes 

Migration directory has been removed

Run diesel migration generate --diff-schema create_db_tables
This will generate both the up.sql and the down.sql files of your migration pre-populated with the relevant SQL.

Run diesel migration run

