# Tags

A way of grouping types together so operations can be implemented over those types.

When creating a tag one must specify the set of functions a type must implement in order to be tagged. When tagging a type the implementations must be provided (unless the function has a default implementation).

Tags can be used as types everywhere as ``#tag`.

## Compound Tags

When using tags, you may want a type to be tagged with more than one tag. Here the compound tags comes in: `#(tag-1, tag-2, ... tag-n)`