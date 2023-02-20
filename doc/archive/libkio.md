# libkio

## Interesting Locations
- `NI::SOUND::RepositoryFactory::createItem(a, itemID, char* domainID)`
	- contains a switch with all repository types
	- if domainID != 'DSIN' ItemFactory::createItem
- `NI::SOUND::ItemFactory::createItem`
