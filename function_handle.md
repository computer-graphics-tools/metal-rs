# MTLFunctionHandle Implementation Notes

This document contains notes about the implementation of MTLFunctionHandle in metal-rs.

## Current Status

The MTLFunctionHandle implementation is currently a placeholder for future Metal API support. 
During implementation, we discovered that the `newFunctionHandle` method does not appear to be 
directly available through the public Metal API in current versions.

## Testing Performed

1. We examined the Objective-C runtime methods available on MTLFunction instances using a test program
2. The `newFunctionHandle` selector was not found on the MTLFunction class
3. Attempting to call this method resulted in an "unrecognized selector sent to instance" error

## Implementation Plan

1. The MTLFunctionHandle type has been implemented with proper structure and traits
2. The MTLFunction.new_function_handle() method panics with an informative message
3. Documentation has been updated to indicate this is a placeholder for future implementation

## Future Work

When the MTLFunctionHandle becomes available in the Metal API (or when the correct method for creating 
function handles is identified), the implementation should be updated accordingly.

Possible approaches:
1. Check newer versions of the Metal framework to see if the API is available there
2. Look for alternative methods to create function handles
3. Update the implementation when Apple's documentation provides more information

Until then, users should use MTLFunction objects directly for shader function references.