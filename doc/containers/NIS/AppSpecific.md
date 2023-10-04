# AppSpecific

AppSpecific acts as a wrapper to notify that its contents have application specific structure, and the reader should check the application and version before determining internal schema.

It has been found to be used in:

- Kontakt Multis
- Kontakt Envelope

It always seems to have (possibly inherits?) a `SubtreeItem` as its child property which carries the `Item`.
