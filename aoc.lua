local M = {}

M.init = function(day, impl, example, input)
    vim.cmd.edit(impl)
    vim.cmd 'Copilot disable' -- no spoilers please
    vim.cmd.vsplit(example)
    vim.cmd.split(input)
    vim.cmd 'wincmd h'

    vim.keymap.set('n', '<leader>a', string.format(':wa | Cargo run --package day%d -- "%s"<CR>', day, input))
    vim.keymap.set('n', '<leader>A', string.format(':wa | Cargo run --package day%d -- "%s"<CR>', day, example))
    vim.keymap.set('n', '<leader>o', string.format(':wa | Cargo run --release --package day%d -- "%s"<CR>', day, input))

    vim.cmd(string.format('DebugExe target/debug/day%d', day))

    function build_and_debug(input) 
        vim.cmd('wa')
        vim.cmd(string.format('!cargo build --package day%d', day))
        if vim.v.shell_error == 0 then
            vim.fn.feedkeys('<CR>') -- get rid of the press any key prompt
            vim.cmd(string.format('DebugArgs %s', input))
            require'dap'.continue()
        end
    end

    vim.keymap.set('n', '<leader>da', function() build_and_debug(input) end)
    vim.keymap.set('n', '<leader>dA', function() build_and_debug(example) end)
end

return M
