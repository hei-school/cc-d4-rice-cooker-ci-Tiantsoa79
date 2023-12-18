# frozen_string_literal: true

require 'minitest/autorun'
require_relative 'rice_cooker'

# Test class for the RiceCooker class
class TestRiceCooker < Minitest::Test
  def setup
    @rice_cooker = RiceCooker.new
  end

  def test_cook_rice
    result = @rice_cooker.cook_rice
    assert_equal true, result
  end

  def test_keep_warm
    result = @rice_cooker.keep_warm
    assert_equal true, result
  end

  def test_steam_cook
    result = @rice_cooker.steam_cook
    assert_equal true, result
  end

  def test_cook_soup
    result = @rice_cooker.cook_soup
    assert_equal true, result
  end

  def test_cook_dessert
    result = @rice_cooker.cook_dessert
    assert_equal true, result
  end
end
