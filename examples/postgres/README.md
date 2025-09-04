# Connecting to the RNACentral Public PostgreSQL Database

This guide provides the steps to connect to the public PostgreSQL database
provided by RNACentral. The database can be used for querying various
RNA-related data.

## Prerequisites

* **PostgreSQL Client:** SQLk requires `psql` to be installed on
your system. If you don't have it, you can install the PostgreSQL package for
your operating system.

## Database Connection Details

The database is a read-only instance, accessible via the following URL:

`postgres://reader:NWDMCE5xdipIjRrp@hh-pgsql-public.ebi.ac.uk:5432/pfmegrnargs`

## Steps

1. **Install SQLk** Follow installation steps.

1. **Open your favourite code editor.** Create a new folder and add the
following files into the root directory:

* ```.env
    DATABASE_URL="postgres://reader:NWDMCE5xdipIjRrp@hh-pgsql-public.ebi.ac.uk:5432/pfmegrnargs"
    ```

* ```.sqlk.toml
  # Environment file path (relative to working directory)
  # These are all of the default options, but configure as you wish
  env_file = ".env"
  
  # Foreign key relationship configuration
  [foreign_keys]
  enabled = true
  
  # Database configuration
  [database]
  # Database URL (can be overridden by .env file or DATABASE_URL environment variable)
  url = "postgres://reader:NWDMCE5xdipIjRrp@hh-pgsql-public.ebi.ac.uk:5432/pfmegrnargs"
  ```

* ```schema.sql
  SELECT
    *
  FROM rnc_accessions
  limit 50;

  SELECT
    *
  FROM rna
  limit 50;
  
  SELECT
    *
  FROM xref
  limit 50;
  
  SELECT
    *
  FROM rnc_database
  limit 50;
  
  SELECT
    *
  FROM rnc_rna_precomputed limit 50;
  ```

3. **Explore the database.**

* If .env, sql file, and .sqlk.toml were placed in root directory

    ```bash
    sqlk
    ```

* To be explicit

    ```bash
    sqlk --file "schema.sql" --env .env 
    ```

### Exploring the database

This database doesn't include foreign_keys and in that case, doesnt show the
full features of SQLk, but let's use it as a quick start into viewing charts.

![Alt text](./../../assets/sqlk-query-viwe.png "SQLk Query View Screen")

After the loading screen, you're presented with a read-only query viewer.
Navigate the home query viewer with `h`/`j`, and execute a highlighted query with
`e`. Exit SQLk with `q`.
**PRO TIP**: The footer gives hints on keybindings, but the bindings are meant
to feel familiar for Vim users.

![Alt text](./../../assets/sqlk-table-view.png "SQLk Table View Screen")

Once the query has executed, you'll see the table view of the formatted results.
Here you can get a lot of insight on your data through different modes provided
by SQLk.

![Alt text](./../../assets/sqlk-table-cell-info.png "SQLk Table Cell Info")

Navigate to any cell in the results table. Get basic cell information using `K`.
This will give context on the column type, and prevalence of the value within
the cell against its column.
_If the cell you were viewing was referencing another table, enter `F` to
get the foreign key refernce in a modal. This isn't application in this demo
since this database doesn't have foreign keys. Reference columns render yellow
text in it's resultset_

![Alt text](./../../assets/sqlk-table-chart.png "SQLk Chart Mode")

Press `c` on any cell to enter Chart mode and view a Bar Chart of its inner
value against its resultset.

 <!-- FIX: Search not rendering text -->
![Alt text](./../../assets/sqlk-search-mode.png "SQLk Search Mode")

Press `/` to enter Search Mode. This allows you to search for a string
throughout the result set. This is not case-sensitive and will find all
instances of the string. Navigate with `n`/`N`.

Press `q` to return to query view. Navigate to run another function, or to
properly exit, press `q` again.
