#!/bin/bash

if [ $# -eq 0 ]; then
    echo "Usage: $0 <program_path>"
    exit 1
fi

PROGRAM_PATH=$1

START=$(date +%s.%N)  # Start time in seconds with nanoseconds
# Your script work starts here

# Example: Execute the program
$PROGRAM_PATH

# Your script work ends here
END=$(date +%s.%N)    # End time in seconds with nanoseconds

# Calculate the time difference in seconds
DIFF=$(echo "$END - $START" | bc)

# Extract hours, minutes, seconds, milliseconds, and nanoseconds
HOURS=$(echo "$DIFF / 3600" | bc)
DIFF=$(echo "$DIFF - $HOURS * 3600" | bc)
MINUTES=$(echo "$DIFF / 60" | bc)
DIFF=$(echo "$DIFF - $MINUTES * 60" | bc)
SECONDS=$(echo "$DIFF" | cut -d '.' -f 1)
MILLISECONDS=$(echo "scale=3; ($DIFF - $SECONDS) * 1000" | bc | sed 's/^\./0./')
NANOSECONDS=$(echo "scale=0; ($DIFF - $SECONDS - $MILLISECONDS / 1000) * 1000000000" | bc)

echo "Execution of $PROGRAM_PATH took $HOURS hours | $MINUTES min | $SECONDS sec | $MILLISECONDS ms | $NANOSECONDS ns"
