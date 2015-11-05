module RubyModule
end

class Ruby
  include RubyModule
  extend RubyModule

  private

  def method(argument)
    begin
      unless
      if true
        "true"
      elsif false
        "false"
      else
        nil
      end
    rescue
      raise
    end

    # comment
    ["ruby"].each do |string|
      variable = 'string'
      another_variable=1
      @instance_variable
      method_call(argument = false, another_argument)
      another_method_call
      hash[:symbol_1234?] = { key: value }
    end
  end
end
