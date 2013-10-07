sources=$(wildcard *.rs)
targets=$(sources:.rs=.run)

all: $(targets)

%.run: %.rs
	rustc $< -o $@
	./$@

clean:
	rm *.run
