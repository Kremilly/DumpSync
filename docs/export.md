# DumpSync Command: Export

To create a database dump, you can use the following command:

```bash
dumpsync export
```

For connectting to a server, read the [Connecting to a Server](../connection) guide.

### Command Breakdown

- **dumpsync**: This is the main command to invoke the DumpSync tool.
- **export**: This subcommand initiates the export process to create a dump of the specified database.

### Options

- **-d my_database**: Specifies the name of the database you want to export. Replace `my_database` with the actual name of your database.
- **-i 3600**: Sets the interval (in seconds) for the dump process. In this example, the interval is set to 3600 seconds (1 hour). You can adjust this value based on your requirements.
- **-f /path/to/**: Indicates the file path where the dump will be saved. Replace `/path/to/` with the desired directory path on your system.
- **--encrypt**: (Optional) Encrypts the dump file using AES-256 encryption. This option requires a password to encrypt and decrypt the dump file.

### Example

```bash
dumpsync export
```

### Encrypt Dumps

To create an encrypted dump file, you can add the `--encrypt` option to the command:

```bash
dumpsync export --encrypt
```

The encryption process use AES-256 encryption and will prompt you to enter a password for the encryption and decryption of the dump file.

### Notes

- The export process will create a dump file of the specified database at the specified interval.
- If you choose to encrypt the dump file, you will need to provide a password during the encryption process.
- Ensure that the specified path for the dump exists and that you have the necessary permissions to write to that directory.
- Adjust the interval according to your backup strategy to ensure that you have up-to-date dumps without overwhelming your database resources.
