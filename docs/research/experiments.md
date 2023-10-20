## Some testing

### Set State/Pilot

- Data order is irrelevant
- Whites (color temp) take precedence over colors
- Dimming takes precedence over color temp
- Colors take precedence over dimming

Due to this it's better to:

- Send stuff separatedly
- Always modify brightness along colors with a Lab color calc to maintain total light output

- setPilot contains a couple more methods than setState.
- setPilot/setState with empty params is an invalid request
- getPilot ignores params (better call it with only the method)
- state param takes int or bool. Int takes any truthy value as true and 0 as false. Wrong types are treated as false
- rgb params all set to 0 gives invalid request
- controlling brightness via color can dim lights further than just brightness alone

- parsing values in colors is strange as it takes floats but maps them strangely:
  - 0.5 is 5
  - 0.50 is 50
  - 1.00 is 100
  - 2.55 is 255
  - 2.56 is 0
  - 5.10 is 255 (it wraps above 2.55)
  - putting more than 2 decimals truncates the number inconsistently

- brightness takes int and float as values or gives invalid params
- brightness parses floats from 0.0 to 1.0 and ints from 0 to 100
- huge ints are truncated and read strangely, 10000 is read as 16.
Large ints are read as hex and truncated to their 2 least significative values:
  - 10000(10) is 2710(16) which sets the value at 10(16) which is 16(10) or 16% of intensity
  - Likewise 60004(10) is EA64(16) which sets the value at 64(16) which is 100(10) or 100% of intensity
  - Same for 20068(10) is 4E64(16) and any other number
What is happening is that is the presence of 00 makes it read the number as hex but only if the whole value vcan be converted to hex and there isn't a 0 on the front. not having the 00 or putting another number or a value not readable as hex in a i16 makes it fail.
