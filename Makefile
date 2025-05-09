# Basic compiler configuration
CC?=gcc
CLANG :=  $(shell $(CC) -v 2>&1 | grep clang)
ifeq ($(CLANG),)
  GCC :=  $(shell $(CC) -v 2>&1 | grep gcc)
endif

WARNING_FLAGS=-Wall -Wextra -Wpedantic -Wshadow
CFLAGS= -O3 -flto=auto -fPIC $(WARNING_FLAGS) -march=native
LINKFLAGS=
MAKE=make

ifneq ($(CLANG),)
    CFLAGS +=  -std=c17 -Wno-bit-int-extension
endif
ifneq ($(GCC),)
    CFLAGS += -std=c23
endif

# Add user EXTRA flags if asked
CFLAGS+=$(EXTRA_CFLAGS)
LINKFLAGS+=$(EXTRA_LINKFLAGS)

# Do we want to activate the debug mode?
ifeq (,$(filter 1,$(DEBUG)))
    CFLAGS += -DNDEBUG
else
    CFLAGS += -g
endif

# Do we want to activate the sanitizers?
ifneq (,$(filter 1,$(USE_SANITIZERS)))
    CFLAGS += -fsanitize=undefined -fsanitize=address -fsanitize=leak
    CFLAGS += -fno-sanitize=signed-integer-overflow,shift-base
    LINKFLAGS += -fsanitize=undefined -fsanitize=address -fsanitize=leak
endif

# Do we want agressive compiler?
ifneq (,$(filter 1,$(AGGRESSIVE)))
    CFLAGS += -Wcast-align
    CLANG :=  $(shell $(CROSS_COMPILE)$(CC) -v 2>&1 | grep clang)
    ifneq ($(CLANG),)
        CFLAGS += -Weverything -Werror \
                -Wno-reserved-id-macro -Wno-padded \
                -Wno-packed -Wno-covered-switch-default \
                -Wno-used-but-marked-unused -Wno-switch-enum
        CFLAGS += -Wno-gnu-zero-variadic-macro-arguments
        CFLAGS += -Wno-unused-macros
        CFLAGS += -Wno-poison-system-directories
        CFLAGS += -Wno-cast-qual
    endif
endif

### Utilitary
comma:= ,

# The function 'get_all_subpaths'
#   takes two folders 'a' and 'b'
#   and retuns the list of all the
#   intermediary folders between 'a'
#   and 'b'.
# For example,
#   $(call get_all_subpaths,a/b,c d e)
# returns
#   "a/b/c a/b/c/d a/b/c/d/e"
define get_all_subpaths
$(shell 
    current=$(1); \
    echo $(current); \
    for folder in $(2) ; do \
        current=$$current/$$folder ;\
        echo $$current; \
    done ;
)
endef

# The function 'get_all_subpaths_from_uri'
#   takes a folder 'a' and a uri 'b'
#   and returns the list of all the folders
#   designed by the URI. A URI is a list of
#   leaf folders and the designed folders are
#   all the intermediary folders between 'a'
#   and the leaf folders.
# For example,
#   $(call get_all_subpaths_from_uri,a/b,c/d,e/f/g)
# returns
#   "a/b a/b/c a/b/c/d a/b/e a/b/e/f a/b/e/f/g"
define get_all_subpaths_from_uri
$(sort $(foreach dir,$(subst $(comma), ,$(2)),$(1) $(call get_all_subpaths,$(1),$(subst /, ,$(dir)))))
endef

# The function 'get_header_files'
#   takes a list of folders and returns
#   the list of all the files with
#   extension .h in the given folders.
define get_header_files
$(foreach dir,$(1),$(wildcard $(dir)/*.h))
endef

# The function 'get_source_files'
#   takes a list of folders and returns
#   the list of all the files with
#   extensions .c, .s, .S in the
#   given folders.
define get_source_files
$(foreach dir,$(1),$(wildcard $(dir)/*.c $(dir)/*.s $(dir)/*.S))
endef

# The function 'get_objects'
#   takes a list of file paths
#   and returns the same list
#   in which the files with extensions
#   .c and .s have been replaced by
#   a file with the same name but with
#   extension .o.
define get_objects
$(patsubst %.s,%.o,$(patsubst %.c,%.o,$(1)))
endef


### CAPSS Path

CFLAGS += -DMEASURE_TIME

# Get the arithmetization type
ifeq ($(CAPSS_PERM),anemoi)
    CAPSS_ARITHM=regular-perm
else ifeq ($(CAPSS_PERM),griffin)
    CAPSS_ARITHM=regular-perm
else ifeq ($(CAPSS_PERM),poseidon)
    CAPSS_ARITHM=sbox-centric
else ifeq ($(CAPSS_PERM),rescueprime)
    CAPSS_ARITHM=regular-perm
else
    $(error Unknown permutation: $(CAPSS_PERM))
endif

ifeq ($(CAPSS_FIELD),bn254)
    CAPSS_LIB=-Lcapss/external/anemoi-rust/target/release -lanemoi
    BUILD_LIB_HASH=cd capss/external/anemoi-rust; cargo build --release
else ifeq ($(CAPSS_FIELD),f64)
    CAPSS_LIB=-Lcapss/external/hash_f64_benchmarks/target/release -lanemoi
    BUILD_LIB_HASH=cd capss/external/hash_f64_benchmarks; cargo build --release
else
    $(error Unknown field: $(CAPSS_FIELD))
endif

ifeq ($(CAPSS_TRADEOFF),short)
    CFLAGS += -DCAPSS_VARIANT=0
else ifeq ($(CAPSS_TRADEOFF),default)
    CFLAGS += -DCAPSS_VARIANT=1
else ifeq ($(CAPSS_TRADEOFF),fast)
    CFLAGS += -DCAPSS_VARIANT=2
else
    $(error Unknown trade-off: $(CAPSS_TRADEOFF))
endif

STATEMENT_PATHS=$(call get_all_subpaths_from_uri,capss,$(CAPSS_ARITHM)/$(CAPSS_PERM) $(CAPSS_FIELD)/$(CAPSS_PERM))
STATEMENT_INCLUDE=$(patsubst %,-I%,$(STATEMENT_PATHS)) -Ismallwood/field -Ismallwood/field/generic
STATEMENT_SRC=$(strip $(call get_source_files,$(STATEMENT_PATHS)))
STATEMENT_OBJ=$(call get_objects,$(STATEMENT_SRC))

LIB=$(CAPSS_LIB)

SW_HASH_PATH=capss/hash-implem

SW_PATH=smallwood
SW_MERKLE_PATH=$(call get_all_subpaths_from_uri,$(SW_PATH),merkle/generic)
SW_DECS_PATH=$(call get_all_subpaths_from_uri,$(SW_PATH),decs)
SW_LVCS_PATH=$(call get_all_subpaths_from_uri,$(SW_PATH),lvcs)
SW_PCS_PATH=$(call get_all_subpaths_from_uri,$(SW_PATH),pcs)
SW_LPPC_PATH=$(call get_all_subpaths_from_uri,$(SW_PATH),lppc)
SW_PIOP_PATH=$(call get_all_subpaths_from_uri,$(SW_PATH),piop)
SW_MAIN_PATH=$(call get_all_subpaths_from_uri,$(SW_PATH),smallwood)
SW_PATHS=$(SW_HASH_PATH) $(SW_MERKLE_PATH) $(SW_DECS_PATH) $(SW_LVCS_PATH) $(SW_PCS_PATH) $(SW_LPPC_PATH) $(SW_PIOP_PATH) $(SW_MAIN_PATH) $(SW_PATH)/benchmark
SW_INCLUDE=$(patsubst %,-I%,$(SW_PATHS))
SW_SRC=$(strip $(call get_source_files,$(SW_PATHS)))
SW_OBJ=$(call get_objects,$(SW_SRC))

INCLUDES=$(STATEMENT_INCLUDE) $(SW_INCLUDE)
OBJECTS=$(STATEMENT_OBJ) $(SW_OBJ)

all: bench

%.o : %.c
	$(CC) -c $(CFLAGS) $(INCLUDES) $< -o $@

libhash:
	$(BUILD_LIB_HASH)

bench:$(OBJECTS) libhash
	$(CC) $(OBJECTS) $(CFLAGS) $(LINKFLAGS) $(INCLUDES) $(LIB) capss/app/bench.c  -lm -o $@

clean:
	rm -f $(OBJECTS)
	rm -f bench
