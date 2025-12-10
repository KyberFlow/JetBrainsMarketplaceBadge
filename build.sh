#Check if Cargo is installed
if ! cargo -v  >/dev/null 2>&1
then
    echo "cargo could not be found"
    exit 1
fi

# Check if the environment variable is set
if [[ -z "${MARKETPLACE_ID}" ]]; then
	echo "You need to set MARKETPLACE_ID"
	exit 1
fi

# Check Debug level
default_debug_level=$DEBUG

if [[ -z "$DEBUG" ]]; then
  echo "No loglevel specify. Fallback to default (DEBUG)"
  default_debug_level="debug"
fi

echo "Cargo is installed, stat to build your "

RUST_LOG=$default_debug_level cargo run --release
