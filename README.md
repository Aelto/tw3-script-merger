# tw3-script-merger
A command line script merger for The Witcher 3

# Usage
The basic usage is the following:
```
$ tw3-script-merger --source "The Witcher 3/content/content0/scripts" --input "The Witcher 3/mods" --output "The Witcher 3/mods/mod0000_MergedFiles/content/scripts"
```

 - `--source` is the location of the vanilla scripts, or the base for the 3-way merge.
 - `--input` is the location of the mods you want to merge, the folder containing the mods.
 - `--output` is the destiniation of the merge.
 - An optional parameter `--clean` can be passed to tell `tw3-script-merger` to clean the output folder before proceeding.

Doing so will result in a new folder being created with all the merges **and the conflicts** in it. You are free to use
whatever method suits you to find the conflicts and resolve them.

___

If you wish to be prompted every time a conflict is detected by the tool, you can pass the optional parameter `--texteditor`.
Here is an example where we're using `code` (visual studio code), note that you may need to path the absolute bath to the
binary if it is not in your PATH:

```
$ tw3-script-merger --source "The Witcher 3/content/content0/scripts" --input "The Witcher 3/mods" --output "The Witcher 3/mods/mod0000_MergedFiles/content/scripts" --texteditor code
```

Doing so will tell the tool to open the supplied text editor for every conflict, and to wait until you resolve them before
proceeding with the merge.

___

If you wish to make a GUI over this cli tool, there is an optional `--json` parameter that will tell `tw3-script-merger`
to output the conflict in a JSON format your code can use. Here is an example of a conflict in the JSON format:
```json
{
  "conflicts":[
    {
      "ours":"\n",
      "original":"\n\t\t\t\r\n\t\t\r\n\t\tburning = (W3Effect_Burning)action.causer;\r\n\t\tif(actorVictim && (((W3IgniEntity)action.causer) || ((W3IgniProjectile)action.causer) || ( burning && burning.IsSignEffect())) )\r\n\t\t{\r\n\t\t\tmin = actorVictim.GetAttributeValue('igni_damage_amplifier');\r\n\t\t\tfinalDamage = finalDamage * (1 + min.valueMultiplicative) + min.valueAdditive;\r\n\t\t}\r\n\t\t\r\n\t\t\r\n",
      "theirs":"\n\t\t\r\n\t\t// Ignore igni_damage_amplifier\r\n",
      "context_before":"\t\tif(dmgInfo.dmgType == theGame.params.DAMAGE_NAME_FIRE && finalDamage > 0)\r\n\t\t\taction.SetDealtFireDamage(true);\r\n\t\t\t\r\n\t\t// Fist attack buff per character level removed\r\n\t\t\r\n\t\tif(playerAttacker && attackAction && playerAttacker.IsHeavyAttack(attackAction.GetAttackName()))\r\n\t\t\tfinalDamage *= 1.833;\r\n",
      "context_after":"\n\t\t\r\n\t\t// Ignore igni_damage_amplifier\r\n\t\t\r\n\t\tif ( theGame.CanLog() )\r\n\t\t{\r\n\t\t\tLogDMHits(\"Single hit damage: initial damage = \" + NoTrailZeros(dmgInfo.dmgVal),
       action);\r\n\t\t\tLogDMHits(\"Single hit damage: attack_power = base: \" + NoTrailZeros(powerMod.valueBase) + \",
       mult: \" + NoTrailZeros(powerMod.va",
      "context_original_size":300
    }
  ],
  "file_name":"damageManagerProcessor.ws",
  "file_path":"output/content/scripts/game/gameplay/damage/damageManagerProcessor.ws",
  "mod_name":"mod0001_shared_lego"
}
```

`tw3-script-merger` will then _watch_ the file and wait until it changes. Your tool is then free to process the conflict
in the way it pleases and write the changes to the file once it is resolved. `tw3-script-merger` will then proceed with
the merge.

# Building
To build `tw3-script-merger` you need to install the rust compiler, once it is done you can run
```
$ cargo build --release
```
This will create a binary at `./target/release/tw3-script-merger`