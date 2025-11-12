
# Infrastructure Bot

This is a discord bot to manage my own infrastructure, it contains usage statistics
about the server like CPU and RAM usage as well as it will be able to query `kubernetes`
information.

> [!WARNING]
> This is not finished, I'm trying my best with school, the infrastructure
> project and trying to find a job, so stay tuned, this will be updated ASAP.

## Objectives For This Bot

This is a list of summarized objectives.

- `kubectl` interface
	- The bot should contain a `kubectl` interface for me to query cluster information in discord.
	- There is going to be a permission system where a role may access filtered data.
- Artifact reader
	- The bot should be able to send in the project channel what failed if failed.
	- Artifacts will be uploaded to S3 for this bot to find keyed by commit hash.
	- There should be a command to list all the artifacts in all the channel where this is being ran.
- Channel register system
	- There should be a command to register a channel as project.
	- When a project is registered it should forward all updates to that channel.
	- This is the base for the artifact reader to know which project is it working with.
	- This should automatically create the web-hook in GitHub.
- Terraform interface
	- I should be able to destroy terraform resources if failed or "reset state" with a command.
	- I want to be able to compare plans and "check what would happen if".

For now this is all I need, but there may be updates in the future.

## License

This is dual licensed with [Apache-2.0](./LICENSE-APACHE) and [Mit](./LICENSE-MIT) at your option.
