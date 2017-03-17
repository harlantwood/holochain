GOBIN = $(value GOPATH)/bin
.PHONY: test deps gx work publish
hc: deps
	go install ./cmd/hc
bs: deps
	go install ./cmd/bs
init: hc
	hc init node@example.com
test: deps
	go get -t
	go test -v ./...||exit 1
testall: test hc init
	hc --debug --verbose clone --force examples/chat   examples-chat   && hc --debug --verbose test examples-chat
	hc --debug --verbose clone --force examples/sample examples-sample && hc --debug --verbose test examples-sample
$(GOBIN)/gx:
	go get github.com/whyrusleeping/gx
$(GOBIN)/gx-go:
	go get github.com/whyrusleeping/gx-go
gx: $(GOBIN)/gx $(GOBIN)/gx-go
	gx install --global
deps: gx
	gx-go rewrite
	go get -d ./...
work:
	gx-go rewrite
publish:
	gx-go rewrite --undo
