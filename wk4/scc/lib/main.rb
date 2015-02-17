RubyVM::InstructionSequence.compile_option = {
  :tailcall_optimization => true,
  :trace_instruction => false
}

require_relative 'scc'

puts SCC.compute_from_file('SCC.txt')
