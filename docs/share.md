# DumpSync Command: Share

The dumpsync share command is used to share on PasteBin the dump or scan result file generated by the DumpSync tool.

```bash
dumpsync share -f <path_to_share_file> --privacy <public|unlisted|private>
```

### Command Breakdown

- **dumpsync**: This is the main command to invoke the DumpSync tool.
- **share**: This subcommand is used to share a dump or scan result file.

### Parameters

- **-f <path_to_share_file>**: Specifies the path to share the file.
- **--privacy <public|unlisted|private>** (Optional|Default: 'unlisted'): Sets the privacy level of the shared file. Options are public, unlisted, or private.

### Example

Share the file `located at path/to/payload.txt`:

```bash
dumpsync share -f path/to/file.txt --privacy public
```

### Privacy Levels

- **Public**: The shared file is visible to everyone.
- **Unlisted**: The shared file is not visible in the public list but can be accessed by anyone with the link.
- **Private**: The shared file is only visible to the user who created it.

### Get the API Key

To share files, you need to get an API key from PasteBin. You can get the API key by creating an account on PasteBin and generating an API key from the account settings. Click [here](https://pastebin.com/doc_api) to get the API key.

### Shareable Files

Formats supported for the share file are:

- SQL
- TXT
- CSV
- JSON
- HTML
