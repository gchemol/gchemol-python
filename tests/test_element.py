#! /usr/bin/env python3
# [[file:~/Workspace/Programming/gchemol-python/pygchemol.note::0608af15-e6ad-4f52-8c8b-b8029951bf94][0608af15-e6ad-4f52-8c8b-b8029951bf94]]
from gchemol import Element

def test_element():
    assert Element('H') == Element(1) == Element.hydrogen == "H"
    assert Element('H').symbol == "H"
    assert Element('H').number == 1
    assert Element.hydrogen.number == 1
    assert Element.hydrogen.symbol == "H"
# 0608af15-e6ad-4f52-8c8b-b8029951bf94 ends here
