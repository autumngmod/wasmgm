local isErr, err = pcall(require, "wasmgm")

if (isErr) then
  print "unable to start wasmgm"
  print "probably you just not installed it, or installed it for other architecture"
  error ("wasmgm unable to start: " .. err)
end