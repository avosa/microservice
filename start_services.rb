#!/usr/bin/env ruby
require 'socket'

# Kill process on a port
def kill_process(port)
  begin
    s = TCPSocket.new('localhost', port)
    s.close
    puts "Port #{port} is in use, trying to kill..."
    pid = `lsof -i :#{port} -t`.chomp
    system("kill -9 #{pid}") unless pid.empty?
  rescue Errno::ECONNREFUSED
    # Port is not in use
  end
end

kill_process(8080)
kill_process(8000)

Dir.chdir('gin_microservice') do
  puts 'Starting Gin microservice...'
  Process.spawn('go run main.go', :out => STDOUT, :err => STDOUT)
end

Dir.chdir('actix_microservice') do
  puts 'Starting Actix microservice...'
  Process.spawn('cargo run', :out => STDOUT, :err => STDOUT)
end

Signal.trap("INT") {
  puts 'Stopping services...'
  kill_process(8080)
  kill_process(8000)
  exit
}

Process.waitall
