# Supporting New WoT Versions

When a new World of Tanks patch is released, the library needs to be updated to support it. This is 
because:

- The schema of existing datastructures may change
    - For example, `OnHealthChanged` method of the `Vehicle` entity may have changes
- Data structures may be added/removed
    - This mean the order of the data structures will change. 
        - For example, if `Vehicle` entity was 2nd  in terms of order in a particular version of WoT, it could be 1st or 3rd in the new WoT patch. Therefore, when we try to parse a `Vehicle` entity, it will fail because the data in the packet we are trying to parse is of a  different entity type

## Dealing With Unknown WoT Versions
In general, if the library is asked to parse a packet from a replay that is not supported (because its too recent). We assume the order and schema of the data structures will match the latest WoT version supported by the library. 

For instance, if the latest  WoT patch supported by the library is `1.20.0` and we are trying to parse a replay from `1.21.0`, the library will try to parse the replay as if it is from `1.20.0`.
