const BUTTONS: [&str; 8] = [
                            "BUTTON_A",
                            "BUTTON_B",
                            "BUTTON_SELECT",
                            "BUTTON_START",
                            "BUTTON_UP",
                            "BUTTON_DOWN",
                            "BUTTON_LEFT",
                            "BUTTON_RIGHT"
                            ];

pub struct Controller
{
    strobe: bool,
    cursor: usize,
    //other: Box<Option<Controller>>,
    _player: u8,
    _buttons: [bool; 8],
}

impl Controller
{
    pub fn new(player: u8) -> Self
    {
        Controller
        {
            strobe: false,
            cursor: 0,
            //other: Box::new(None),
            _player: player,
            _buttons: [false,false,false,false,false,false,false,false],
        }
    }
    pub fn update(&mut self, button: &str, is_pressed: bool)
    {
        let index: usize= BUTTONS.iter().position(|&b| b == button).unwrap();
        self._buttons[index] = is_pressed;
    }
    pub fn on_read(&mut self) -> u8
    {
        if self.strobe
        {
            return if self._buttons[0] {1} else{0};
        }
        if self.cursor >= 8 { return 1; }

        let value: u8 = if self._buttons[self.cursor] {1} else {0};
        self.cursor += 1;
        return value;
    }
    pub fn on_write(&mut self, value: u8)
    {
        if self._player != 1 {return;}

        let strobe_on: bool = (value & 1) == 1;

        if strobe_on
        {
            self.strobe = true;
            self.cursor = 0;
            /*
            if self.other
            {
                self.other.strobe = true;
                self.other.cursor = 0;
            }
            */
        }
        else
        {
            self.strobe = false;
            self.cursor = 0;
            /*
            if self.other
            {
                self.other.strobe = false;
                self.other.cursor = 0;
            }
            */
        }
    }
}